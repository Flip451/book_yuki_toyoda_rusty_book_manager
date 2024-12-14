use chrono::DateTime;
use chrono::Utc;
use derive_getters::Dissolve;
use thiserror::Error;
use uuid::Uuid;

pub mod event;

use crate::impl_entity;
use crate::tuple_value_object_with_simple_error;

use super::checkout::CheckoutId;
use super::user::BookOwner;
use super::user::CheckoutUser;

tuple_value_object_with_simple_error!(BookId, Uuid, BookIdError);
tuple_value_object_with_simple_error!(Title, String, TitleError);
tuple_value_object_with_simple_error!(Author, String, AuthorError);
tuple_value_object_with_simple_error!(Isbn, String, IsbnError);
tuple_value_object_with_simple_error!(Description, String, DescriptionError);

#[cfg(not(feature = "test-utils"))]
#[derive(Debug, derive_new::new, Dissolve)]
pub struct Book {
    book_id: BookId,
    title: Title,
    author: Author,
    isbn: Isbn,
    description: Description,
    owner: BookOwner,
    checkout: Option<Checkout>,
}

#[cfg(feature = "test-utils")]
#[derive(Debug, derive_new::new, Dissolve)]
pub struct Book {
    pub book_id: BookId,
    pub title: Title,
    pub author: Author,
    pub isbn: Isbn,
    pub description: Description,
    pub owner: BookOwner,
    pub checkout: Option<Checkout>,
}

impl_entity!(Book, book_id, BookId);

#[derive(Debug, Dissolve)]
pub struct Checkout {
    pub checkout_id: CheckoutId,
    pub checked_out_by: CheckoutUser,
    pub checked_out_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum BookError {
    #[error("invalid book id: {0}")]
    InvalidBookId(#[from] BookIdError),

    #[error("invalid title: {0}")]
    InvalidTitle(#[from] TitleError),

    #[error("invalid author: {0}")]
    InvalidAuthor(#[from] AuthorError),

    #[error("invalid isbn: {0}")]
    InvalidIsbn(#[from] IsbnError),

    #[error("invalid description: {0}")]
    InvalidDescription(#[from] DescriptionError),
}

pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}
