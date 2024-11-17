use super::{Author, Description, Isbn, Title};

pub struct CreateBook {
    pub title: Title,
    pub author: Author,
    pub isbn: Isbn,
    pub description: Description,
}
