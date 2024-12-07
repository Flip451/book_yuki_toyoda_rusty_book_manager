use kernel::model::{
    book::{AuthorError, Book, BookIdError, DescriptionError, IsbnError, TitleError},
    user::{BookOwner, UserIdError, UserNameError},
};
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

impl TryFrom<BookRow> for Book {
    type Error = BookRowError;

    fn try_from(
        BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
            owner_id,
            owner_name,
        }: BookRow,
    ) -> Result<Self, Self::Error> {
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
        ))
    }
}

pub struct PagenatedBookRow {
    pub total: i64,
    pub book_id: Uuid,
}
