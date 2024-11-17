use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::book::{event::CreateBook, Book, BookId},
    repository::book::BookRepository,
};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> Result<()> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        todo!()
    }

    async fn find_by_id(&self, id: BookId) -> Result<Option<Book>> {
        todo!()
    }
}
