use kernel::model::{
    book::{
        event::CreateBook, Author, AuthorError, Book, Description, DescriptionError, Isbn,
        IsbnError, Title, TitleError,
    },
    value_object::ValueObject,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl TryFrom<CreateBookRequest> for CreateBook {
    type Error = CreateBookRequestError;

    fn try_from(request: CreateBookRequest) -> Result<Self, Self::Error> {
        Ok(CreateBook::new(
            Title::try_from(request.title)?,
            Author::try_from(request.author)?,
            Isbn::try_from(request.isbn)?,
            Description::try_from(request.description)?,
        ))
    }
}

#[derive(Debug, Error)]
pub enum CreateBookRequestError {
    #[error("invalid title: {0}")]
    InvalidTitle(#[from] TitleError),

    #[error("invalid author: {0}")]
    InvalidAuthor(#[from] AuthorError),

    #[error("invalid isbn: {0}")]
    InvalidIsbn(#[from] IsbnError),

    #[error("invalid description: {0}")]
    InvalidDescription(#[from] DescriptionError),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        let (book_id, title, author, isbn, description) = book.dissolve();

        BookResponse {
            book_id: book_id.into_inner(),
            title: title.into_inner(),
            author: author.into_inner(),
            isbn: isbn.into_inner(),
            description: description.into_inner(),
        }
    }
}
