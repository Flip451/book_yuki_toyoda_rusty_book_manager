use kernel::model::{
    book::{event::CreateBook, Author, Book, Description, Isbn, Title},
    value_object::ValueObject,
};
use serde::{Deserialize, Serialize};
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
    type Error = anyhow::Error;

    fn try_from(request: CreateBookRequest) -> Result<Self, Self::Error> {
        Ok(CreateBook::new(
            Title::try_from(request.title)?,
            Author::try_from(request.author)?,
            Isbn::try_from(request.isbn)?,
            Description::try_from(request.description)?,
        ))
    }
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
    fn from(
        Book {
            book_id,
            title,
            author,
            isbn,
            description,
        }: Book,
    ) -> Self {
        BookResponse {
            book_id: book_id.into_inner(),
            title: title.into_inner(),
            author: author.into_inner(),
            isbn: isbn.into_inner(),
            description: description.into_inner(),
        }
    }
}
