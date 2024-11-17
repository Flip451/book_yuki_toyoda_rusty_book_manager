use kernel::model::book::{Author, Book, BookId, Description, Isbn, Title};
use uuid::Uuid;

pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<BookRow> for Book {
    fn from(
        BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        }: BookRow,
    ) -> Self {
        Book::new(
            BookId::new(book_id),
            Title::new(title),
            Author::new(author),
            Isbn::new(isbn),
            Description::new(description),
        )
    }
}
