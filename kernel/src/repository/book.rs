use async_trait::async_trait;
use thiserror::Error;

use crate::model::{
    book::{
        event::{CreateBook, DeleteBook, UpdateBook},
        Book, BookId, BookListOptions,
    },
    list::PaginatedList,
    user::UserId,
};

#[mockall::automock]
#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(&self, event: CreateBook, owner_id: UserId) -> BookRepositoryResult<()>;
    async fn find_all(&self, options: BookListOptions)
        -> BookRepositoryResult<PaginatedList<Book>>;
    async fn find_by_id(&self, id: &BookId) -> BookRepositoryResult<Option<Book>>;
    async fn update(&self, event: UpdateBook) -> BookRepositoryResult<()>;
    async fn delete(&self, event: DeleteBook) -> BookRepositoryResult<()>;
}

#[derive(Debug, Error)]
pub enum BookRepositoryError {
    #[error("saved entity is invalid: {0}")]
    InvalidSavedEntity(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("unexpected error occurred: {0}")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("book not found: {0}")]
    NotFound(BookId),

    #[error("no resource was affected: {0}")]
    NoResourceAffected(String),
}

pub type BookRepositoryResult<T> = Result<T, BookRepositoryError>;
