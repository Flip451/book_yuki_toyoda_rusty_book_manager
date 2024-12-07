use crate::model::user::UserId;

use super::{Author, BookId, Description, Isbn, Title};

pub struct CreateBook {
    pub title: Title,
    pub author: Author,
    pub isbn: Isbn,
    pub description: Description,
}

pub struct UpdateBook {
    pub book_id: BookId,
    pub title: Title,
    pub author: Author,
    pub isbn: Isbn,
    pub description: Description,
    pub requested_by: UserId,
}

pub struct DeleteBook {
    pub book_id: BookId,
    pub requested_by: UserId,
}
