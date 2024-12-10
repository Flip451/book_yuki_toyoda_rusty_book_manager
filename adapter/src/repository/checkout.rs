use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        book::BookId,
        checkout::{
            event::{CreateCheckout, UpdateReturned},
            Checkout,
        },
        user::UserId,
        value_object::ValueObject,
    },
    repository::checkout::{CheckoutRepository, CheckoutRepositoryError, CheckoutRepositoryResult},
};
use uuid::Uuid;

use crate::database::{
    model::checkout::{CheckoutRow, CheckoutStateRow, ReturnedCheckoutRow},
    ConnectionPool,
};

#[derive(new)]
pub struct CheckoutRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl CheckoutRepository for CheckoutRepositoryImpl {
    async fn create(&self, event: CreateCheckout) -> CheckoutRepositoryResult<()> {
        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| CheckoutRepositoryError::Transaction(e.into()))?;

        // トランザクション分離レベルをSERIALIZABLEに設定
        self.set_transaction_serializable(&mut tx).await?;

        // 事前のチェックとして以下を調べる：
        // - 指定の蔵書IDを持つ蔵書が存在するか
        // - 存在した場合、蔵書がすでに貸出中でなないか
        //
        // 上記の両方が YES の場合、このブロックより後の処理に進む
        {
            let res = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id AS "checkout_id?: Uuid",
                        NULL AS "user_id?: Uuid"
                    FROM books b
                    LEFT OUTER JOIN checkouts c USING (book_id)
                    WHERE b.book_id = $1;
                "#,
                event.book_id.inner_ref(),
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

            match res {
                None => return Err(CheckoutRepositoryError::BookNotFound(event.book_id.clone())),
                Some(CheckoutStateRow {
                    checkout_id: Some(_),
                    ..
                }) => {
                    return Err(CheckoutRepositoryError::BookAlreadyCheckedOut(
                        event.book_id.clone(),
                    ))
                }
                _ => {}
            }
        }

        let res = sqlx::query!(
            r#"
                INSERT INTO checkouts (book_id, user_id, checked_out_at)
                VALUES ($1, $2, $3)
            "#,
            event.book_id.inner_ref(),
            event.checked_out_by.inner_ref(),
            event.checked_out_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

        if res.rows_affected() < 1 {
            return Err(CheckoutRepositoryError::NoResourceAffected(
                "No checkouts record has been inserted.".to_string(),
            ));
        }

        tx.commit()
            .await
            .map_err(|e| CheckoutRepositoryError::Transaction(e.into()))?;

        Ok(())
    }

    async fn find_unreturned_all(&self) -> CheckoutRepositoryResult<Vec<Checkout>> {
        let checkouts = sqlx::query_as!(
            CheckoutRow,
            r#"
            SELECT
                c.checkout_id,
                c.user_id,
                c.checked_out_at,
                b.book_id,
                b.title,
                b.author,
                b.isbn
            FROM checkouts c
            INNER JOIN books b ON c.book_id = b.book_id
            ORDER BY checked_out_at ASC
            "#,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

        checkouts
            .into_iter()
            .map(Checkout::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| CheckoutRepositoryError::InvalidSavedEntity(e.into()))
    }

    async fn find_unreturned_by_user_id(
        &self,
        user_id: &UserId,
    ) -> CheckoutRepositoryResult<Vec<Checkout>> {
        let checkouts = sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.user_id,
                    c.checked_out_at,
                    b.book_id,
                    b.title,
                    b.author,
                    b.isbn
                FROM checkouts c
                INNER JOIN books b USING (book_id)
                WHERE c.user_id = $1
                ORDER BY checked_out_at ASC
            "#,
            user_id.inner_ref(),
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

        checkouts
            .into_iter()
            .map(Checkout::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| CheckoutRepositoryError::InvalidSavedEntity(e.into()))
    }

    async fn find_history_by_book_id(
        &self,
        book_id: &BookId,
    ) -> CheckoutRepositoryResult<Vec<Checkout>> {
        let checkouts = sqlx::query_as!(
            ReturnedCheckoutRow,
            r#"
                SELECT
                    rc.checkout_id,
                    rc.user_id,
                    rc.checked_out_at,
                    rc.returned_at,
                    b.book_id,
                    b.title,
                    b.author,
                    b.isbn
                FROM returned_checkouts rc
                INNER JOIN books b USING (book_id)
                WHERE rc.book_id = $1
                ORDER BY checked_out_at DESC
            "#,
            book_id.inner_ref(),
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

        let checking_out = sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.user_id,
                    c.checked_out_at,
                    b.book_id,
                    b.title,
                    b.author,
                    b.isbn
                FROM books b
                INNER JOIN checkouts c USING (book_id)
                WHERE b.book_id = $1;
            "#,
            book_id.inner_ref(),
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?
        .map(Checkout::try_from)
        .transpose()
        .map_err(|e| CheckoutRepositoryError::InvalidSavedEntity(e.into()))?;

        let mut checkouts = checkouts
            .into_iter()
            .map(Checkout::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| CheckoutRepositoryError::InvalidSavedEntity(e.into()))?;

        if let Some(checking_out) = checking_out {
            checkouts.insert(0, checking_out);
        }

        Ok(checkouts)
    }

    async fn update_returned(&self, event: UpdateReturned) -> CheckoutRepositoryResult<()> {
        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| CheckoutRepositoryError::Transaction(e.into()))?;

        // トランザクション分離レベルをSERIALIZABLEに設定
        self.set_transaction_serializable(&mut tx).await?;

        // 以下を確認してから後続の処理を行う
        // - 与えられた book_id を持つ蔵書が存在するか
        // - 仮に存在するとしたら、その蔵書は貸し出し中か（checkouts テーブルにレコードが存在するか）
        //   - 貸し出し中なら、その貸し出しの ID は与えられた checkout_id に一致するか
        //   - また、借主は与えられた user_id（returned_by） に一致するか
        {
            let res = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id AS "checkout_id?: Uuid",
                        c.user_id AS "user_id?: Uuid"
                    FROM books b
                    LEFT OUTER JOIN checkouts c USING (book_id)
                    WHERE b.book_id = $1;
                "#,
                event.book_id.inner_ref(),
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

            match res {
                None => return Err(CheckoutRepositoryError::BookNotFound(event.book_id.clone())),
                Some(CheckoutStateRow {
                    book_id: _b,
                    checkout_id: Some(c),
                    user_id: Some(u),
                }) if (&c, &u)
                    != (event.checkout_id.inner_ref(), event.returned_by.inner_ref()) =>
                {
                    return Err(CheckoutRepositoryError::CannotReturn(
                        event.book_id.clone(),
                        event.returned_by.clone(),
                        event.checkout_id.clone(),
                    ))
                }
                _ => {}
            }
        }

        let res = sqlx::query!(
            r#"
                INSERT INTO returned_checkouts (
                    checkout_id,
                    book_id,
                    user_id,
                    checked_out_at,
                    returned_at
                )
                SELECT
                    c.checkout_id,
                    c.book_id,
                    c.user_id,
                    c.checked_out_at,
                    $2
                FROM checkouts c
                WHERE c.checkout_id = $1
                ;
            "#,
            event.checkout_id.inner_ref(),
            event.returned_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

        if res.rows_affected() < 1 {
            return Err(CheckoutRepositoryError::NoResourceAffected(
                "No returned checkouts record has been inserted.".to_string(),
            ));
        }

        let res = sqlx::query!(
            r#"
                DELETE FROM checkouts WHERE checkout_id = $1;
            "#,
            event.checkout_id.inner_ref(),
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

        if res.rows_affected() < 1 {
            return Err(CheckoutRepositoryError::NoResourceAffected(
                "No checkouts record has been deleted.".to_string(),
            ));
        }

        tx.commit()
            .await
            .map_err(|e| CheckoutRepositoryError::Transaction(e.into()))?;

        Ok(())
    }
}

impl CheckoutRepositoryImpl {
    async fn set_transaction_serializable(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> CheckoutRepositoryResult<()> {
        sqlx::query!("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut **tx)
            .await
            .map_err(|e| CheckoutRepositoryError::Unexpected(e.into()))?;

        Ok(())
    }
}
