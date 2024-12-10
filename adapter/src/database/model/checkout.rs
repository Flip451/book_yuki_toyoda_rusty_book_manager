use kernel::model::{
    book::{AuthorError, BookIdError, IsbnError, TitleError},
    checkout::{Checkout, CheckoutBook, CheckoutIdError},
    user::UserIdError,
};
use sqlx::types::chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

// 貸出状態を確認するための型
// 蔵書が存在する場合はこの型にはまるレコードが存在する
// 貸出中の場合は checkout_id と user_id がSomeになる
// 貸し出し中でない場合は checkout_id と user_id がNoneになる
pub(crate) struct CheckoutStateRow {
    pub book_id: Uuid,
    pub checkout_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

pub(crate) struct CheckoutRow {
    pub checkout_id: Uuid,
    pub user_id: Uuid,
    pub checked_out_at: DateTime<Utc>,
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl TryFrom<CheckoutRow> for Checkout {
    type Error = CheckoutRowError;

    fn try_from(value: CheckoutRow) -> Result<Self, Self::Error> {
        let CheckoutRow {
            checkout_id,
            user_id,
            checked_out_at,
            book_id,
            title,
            author,
            isbn,
        } = value;

        Ok(Checkout {
            checkout_id: checkout_id.try_into()?,
            checked_out_by: user_id.try_into()?,
            checked_out_at,
            returned_at: None,
            book: CheckoutBook {
                book_id: book_id.try_into()?,
                title: title.try_into()?,
                author: author.try_into()?,
                isbn: isbn.try_into()?,
            },
        })
    }
}

#[derive(Debug, Error)]
pub enum CheckoutRowError {
    #[error("saved checkout id is invalid: {0}")]
    InvalidCheckoutId(#[from] CheckoutIdError),

    #[error("saved user id is invalid: {0}")]
    InvalidUserId(#[from] UserIdError),

    #[error("saved book id is invalid: {0}")]
    InvalidBookId(#[from] BookIdError),

    #[error("saved title is invalid: {0}")]
    InvalidTitle(#[from] TitleError),

    #[error("saved author is invalid: {0}")]
    InvalidAuthor(#[from] AuthorError),

    #[error("saved isbn is invalid: {0}")]
    InvalidIsbn(#[from] IsbnError),
}

pub(crate) struct ReturnedCheckoutRow {
    pub checkout_id: Uuid,
    pub user_id: Uuid,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl TryFrom<ReturnedCheckoutRow> for Checkout {
    type Error = ReturnedCheckoutRowError;

    fn try_from(value: ReturnedCheckoutRow) -> Result<Self, Self::Error> {
        let ReturnedCheckoutRow {
            checkout_id,
            user_id,
            checked_out_at,
            returned_at,
            book_id,
            title,
            author,
            isbn,
        } = value;

        Ok(Checkout {
            checkout_id: checkout_id.try_into()?,
            checked_out_by: user_id.try_into()?,
            checked_out_at,
            returned_at: Some(returned_at),
            book: CheckoutBook {
                book_id: book_id.try_into()?,
                title: title.try_into()?,
                author: author.try_into()?,
                isbn: isbn.try_into()?,
            },
        })
    }
}

#[derive(Debug, Error)]
pub enum ReturnedCheckoutRowError {
    #[error("saved checkout id is invalid: {0}")]
    InvalidCheckoutId(#[from] CheckoutIdError),

    #[error("saved user id is invalid: {0}")]
    InvalidUserId(#[from] UserIdError),

    #[error("saved book id is invalid: {0}")]
    InvalidBookId(#[from] BookIdError),

    #[error("saved title is invalid: {0}")]
    InvalidTitle(#[from] TitleError),

    #[error("saved author is invalid: {0}")]
    InvalidAuthor(#[from] AuthorError),

    #[error("saved isbn is invalid: {0}")]
    InvalidIsbn(#[from] IsbnError),
}
