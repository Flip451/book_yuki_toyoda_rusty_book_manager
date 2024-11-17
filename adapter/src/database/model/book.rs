use kernel::model::book::{Author, Book, BookId, Description, Isbn, Title};
use uuid::Uuid;

struct BookRow {
    book_id: Uuid,
    title: String,
    author: String,
    isbn: String,
    description: String,
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
