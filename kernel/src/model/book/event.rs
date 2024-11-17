use derive_new::new;

use super::{Author, Description, Isbn, Title};

#[derive(new)]
pub struct CreateBook {
    pub title: Title,
    pub author: Author,
    pub isbn: Isbn,
    pub description: Description,
}
