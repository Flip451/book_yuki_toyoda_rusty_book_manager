use std::collections::HashMap;

use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::event::{DeleteBook, UpdateBook};
use kernel::model::book::{BookIdError, BookListOptions, Checkout};
use kernel::model::list::PaginatedList;
use kernel::model::user::UserId;
use kernel::model::value_object::ValueObject;
use kernel::repository::book::{BookRepositoryError, BookRepositoryResult};
use kernel::{
    model::book::{event::CreateBook, Book, BookId},
    repository::book::BookRepository,
};

use crate::database::model::book::{
    BookCheckoutRow, BookCheckoutRowError, BookRow, BookRowError, PagenatedBookRow,
};
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook, owner_id: UserId) -> BookRepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO books (title, author, isbn, description, user_id)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            event.title.inner_ref(),
            event.author.inner_ref(),
            event.isbn.inner_ref(),
            event.description.inner_ref(),
            owner_id.inner_ref(),
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;
        Ok(())
    }

    async fn find_all(
        &self,
        options: BookListOptions,
    ) -> BookRepositoryResult<PaginatedList<Book>> {
        let BookListOptions { limit, offset } = options;

        let rows = sqlx::query_as!(
            PagenatedBookRow,
            r#"
                SELECT
                    COUNT(*) OVER() AS "total!",
                    book_id
                FROM books
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;

        let total = rows.first().map(|r| r.total).unwrap_or_default(); // レコードが一つもないときは total は 0 にする

        let book_ids = rows.into_iter().map(|r| r.book_id).collect::<Vec<_>>();

        let rows = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id,
                    b.title,
                    b.author,
                    b.isbn,
                    b.description,
                    u.user_id AS owner_id,
                    u.name AS owner_name
                FROM books b
                INNER JOIN users u USING(user_id)
                WHERE b.book_id IN (SELECT * FROM UNNEST($1::uuid[]))
                ORDER BY b.created_at DESC
            "#,
            &book_ids,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(e.into()))?;

        let book_ids = rows
            .iter()
            .map(|book| {
                book.book_id
                    .try_into()
                    .map_err(|e: BookIdError| BookRepositoryError::InvalidSavedEntity(e.into()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut checkouts = self.find_checkouts(&book_ids).await?;

        let items = rows
            .into_iter()
            .map(|row| {
                let checkout = checkouts.remove(&row.book_id.try_into()?);
                row.try_into_book(checkout)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e: BookRowError| BookRepositoryError::InvalidSavedEntity(e.into()))?;

        Ok(PaginatedList {
            total,
            limit,
            offset,
            items,
        })
    }

    async fn find_by_id(&self, book_id: &BookId) -> BookRepositoryResult<Option<Book>> {
        let row = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id,
                    b.title,
                    b.author,
                    b.isbn,
                    b.description,
                    u.user_id AS owner_id,
                    u.name AS owner_name
                FROM books b
                INNER JOIN users u USING(user_id)
                WHERE b.book_id = $1
            "#,
            book_id.inner_ref(),
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;

        // row.map(|row| row.try_into())
        //     .transpose()
        //     .map_err(|e: BookRowError| BookRepositoryError::InvalidSavedEntity(e.into()))

        match row {
            Some(r) => {
                let book_id: BookId = r
                    .book_id
                    .try_into()
                    .map_err(|e: BookIdError| BookRepositoryError::InvalidSavedEntity(e.into()))?;
                let checkout = self
                    .find_checkouts(&[book_id.clone()])
                    .await?
                    .remove(&book_id);
                let book = r
                    .try_into_book(checkout)
                    .map_err(|e: BookRowError| BookRepositoryError::InvalidSavedEntity(e.into()))?;
                Ok(Some(book))
            }
            None => Ok(None),
        }
    }

    async fn update(&self, event: UpdateBook) -> BookRepositoryResult<()> {
        let res = sqlx::query!(
            r#"
                UPDATE books
                SET
                    title = $1,
                    author = $2,
                    isbn = $3,
                    description = $4
                WHERE book_id = $5
                AND user_id = $6
            "#,
            event.title.inner_ref(),
            event.author.inner_ref(),
            event.isbn.inner_ref(),
            event.description.inner_ref(),
            event.book_id.inner_ref(),
            event.requested_by.inner_ref(),
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;

        if res.rows_affected() < 1 {
            return Err(BookRepositoryError::NoResourceAffected(
                "No books record has been updated.".to_string(),
            ));
        }

        Ok(())
    }

    async fn delete(&self, event: DeleteBook) -> BookRepositoryResult<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM books WHERE book_id = $1 AND user_id = $2
            "#,
            event.book_id.inner_ref(),
            event.requested_by.inner_ref(),
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;

        if res.rows_affected() < 1 {
            return Err(BookRepositoryError::NoResourceAffected(
                "No books record has been deleted.".to_string(),
            ));
        }

        Ok(())
    }
}

impl BookRepositoryImpl {
    async fn find_checkouts(
        &self,
        book_ids: &[BookId],
    ) -> BookRepositoryResult<HashMap<BookId, Checkout>> {
        let book_ids = book_ids.iter().map(|b| *b.inner_ref()).collect::<Vec<_>>();

        let res = sqlx::query_as!(
            BookCheckoutRow,
            r#"
                SELECT
                    checkout_id,
                    book_id,
                    user_id,
                    u.name AS user_name,
                    checked_out_at
                FROM checkouts
                INNER JOIN users u USING(user_id)
                WHERE book_id IN (SELECT * FROM UNNEST($1::uuid[]))
            "#,
            &book_ids[..],
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;

        let map =
            res.into_iter()
                .map(|r| {
                    let book_id = r.book_id.try_into().map_err(|e: BookIdError| {
                        BookRepositoryError::InvalidSavedEntity(e.into())
                    })?;
                    let checkout = r.try_into().map_err(|e: BookCheckoutRowError| {
                        BookRepositoryError::InvalidSavedEntity(e.into())
                    })?;
                    Ok((book_id, checkout))
                })
                .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use kernel::{
        model::{
            book::{Author, Description, Isbn, Title},
            user::{event::CreateUser, Password, UserEmail, UserName},
        },
        repository::user::UserRepository,
    };

    use crate::repository::user::UserRepositoryImpl;

    use super::*;

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO roles(name) VALUES ('Admin'), ('User');
            "#,
        )
        .execute(&pool)
        .await?;

        let user_repo = UserRepositoryImpl::new(ConnectionPool::new(pool.clone()));

        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));

        let user = CreateUser {
            name: UserName::try_from("test user".to_string())?,
            email: "test@example.com".parse::<UserEmail>()?,
            password: Password::try_from("password".to_string())?,
        };

        let user = user_repo.create(user).await?;

        let book = CreateBook {
            title: Title::try_from("test title".to_string())?,
            author: Author::try_from("test author".to_string())?,
            isbn: Isbn::try_from("test isbn".to_string())?,
            description: Description::try_from("test description".to_string())?,
        };

        repo.create(book, user.user_id().clone()).await?;

        let options = BookListOptions {
            limit: 10,
            offset: 0,
        };

        let res = repo.find_all(options).await?;
        assert_eq!(res.items.len(), 1);
        assert_eq!(res.total, 1);
        assert_eq!(res.limit, 10);
        assert_eq!(res.offset, 0);

        let book_id = &res.items[0].book_id;
        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        let Book {
            book_id: id,
            title,
            author,
            isbn,
            description,
            owner,
            ..
        } = res.ok_or(anyhow::anyhow!("book not found"))?;

        assert_eq!(book_id.inner_ref(), id.inner_ref());
        assert_eq!(title.inner_ref(), "test title");
        assert_eq!(author.inner_ref(), "test author");
        assert_eq!(isbn.inner_ref(), "test isbn");
        assert_eq!(description.inner_ref(), "test description");
        assert_eq!(owner.user_id, *user.user_id());
        assert_eq!(owner.user_name, "test user".to_string().try_into()?);

        Ok(())
    }
}
