use kernel::model::{
    book::{AuthorError, Book, BookIdError, Checkout, DescriptionError, IsbnError, TitleError},
    checkout::CheckoutIdError,
    user::{BookOwner, CheckoutUser, UserIdError, UserNameError},
};
use sqlx::types::chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner_id: Uuid,
    pub owner_name: String,
}

#[derive(Debug, Error)]
pub enum BookRowError {
    #[error("saved book id is invalid: {0}")]
    InvalidBookId(#[from] BookIdError),

    #[error("saved book title is invalid: {0}")]
    InvalidBookTitle(#[from] TitleError),

    #[error("saved book author is invalid: {0}")]
    InvalidBookAuthor(#[from] AuthorError),

    #[error("saved book isbn is invalid: {0}")]
    InvalidBookIsbn(#[from] IsbnError),

    #[error("saved book description is invalid: {0}")]
    InvalidBookDescription(#[from] DescriptionError),

    #[error("saved book owner id is invalid: {0}")]
    InvalidBookOwnerId(#[from] UserIdError),

    #[error("saved book owner name is invalid: {0}")]
    InvalidBookOwnerName(#[from] UserNameError),
}

impl BookRow {
    pub fn try_into_book(self, checkout: Option<Checkout>) -> Result<Book, BookRowError> {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
            owner_id,
            owner_name,
        } = self;

        let book_owner = BookOwner {
            user_id: owner_id.try_into()?,
            user_name: owner_name.try_into()?,
        };

        Ok(Book::new(
            book_id.try_into()?,
            title.try_into()?,
            author.try_into()?,
            isbn.try_into()?,
            description.try_into()?,
            book_owner,
            checkout,
        ))
    }
}

pub struct PagenatedBookRow {
    pub total: i64,
    pub book_id: Uuid,
}

pub struct BookCheckoutRow {
    pub checkout_id: Uuid,
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub checked_out_at: DateTime<Utc>,
}

impl TryFrom<BookCheckoutRow> for Checkout {
    type Error = BookCheckoutRowError;

    fn try_from(
        BookCheckoutRow {
            checkout_id,
            book_id: _,
            user_id,
            user_name,
            checked_out_at,
        }: BookCheckoutRow,
    ) -> Result<Self, Self::Error> {
        Ok(Checkout {
            checkout_id: checkout_id.try_into()?,
            checked_out_by: CheckoutUser {
                user_id: user_id.try_into()?,
                user_name: user_name.try_into()?,
            },
            checked_out_at,
        })
    }
}

#[derive(Debug, Error)]
pub enum BookCheckoutRowError {
    #[error("saved book checkout id is invalid: {0}")]
    InvalidBookCheckoutId(#[from] CheckoutIdError),

    #[error("saved book checkout user id is invalid: {0}")]
    InvalidBookCheckoutUserId(#[from] UserIdError),

    #[error("saved book checkout user name is invalid: {0}")]
    InvalidBookCheckoutUserName(#[from] UserNameError),
}
