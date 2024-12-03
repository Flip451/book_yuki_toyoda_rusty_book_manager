use async_trait::async_trait;
use thiserror::Error;

use crate::model::book::{event::CreateBook, Book, BookId};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(&self, event: CreateBook) -> BookRepositoryResult<()>;
    async fn find_all(&self) -> BookRepositoryResult<Vec<Book>>;
    async fn find_by_id(&self, id: &BookId) -> BookRepositoryResult<Option<Book>>;
}

#[derive(Debug, Error)]
pub enum BookRepositoryError {
    #[error("saved entity is invalid: {0}")]
    InvalidSavedEntity(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("unexpected error occurred: {0}")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub type BookRepositoryResult<T> = Result<T, BookRepositoryError>;
