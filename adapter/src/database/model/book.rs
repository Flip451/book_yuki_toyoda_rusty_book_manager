use kernel::model::book::{Author, Book, BookId, Description, Isbn, Title};
use uuid::Uuid;

pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl TryFrom<BookRow> for Book {
    type Error = anyhow::Error;

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
            BookId::try_from(book_id)?,
            Title::try_from(title)?,
            Author::try_from(author)?,
            Isbn::try_from(isbn)?,
            Description::try_from(description)?,
        ))
    }
}
