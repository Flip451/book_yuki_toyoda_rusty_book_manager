use kernel::model::book::{Book, BookError};
use uuid::Uuid;

pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

// TODO: Add BookRowError

impl TryFrom<BookRow> for Book {
    type Error = BookError;

    fn try_from(
        BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        }: BookRow,
    ) -> Result<Self, Self::Error> {
        Ok(Book::new(
            book_id.try_into()?,
            title.try_into()?,
            author.try_into()?,
            isbn.try_into()?,
            description.try_into()?,
        ))
    }
}
