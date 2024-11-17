use uuid::Uuid;

pub mod event;

use crate::impl_entity;
use crate::tuple_value_object_without_error;

tuple_value_object_without_error!(BookId, Uuid);
tuple_value_object_without_error!(Title, String);
tuple_value_object_without_error!(Author, String);
tuple_value_object_without_error!(Isbn, String);
tuple_value_object_without_error!(Description, String);

pub struct Book {
    pub id: BookId,
    pub title: Title,
    pub author: Author,
    pub isbn: Isbn,
    pub description: Description,
}

impl_entity!(Book, id, BookId);
