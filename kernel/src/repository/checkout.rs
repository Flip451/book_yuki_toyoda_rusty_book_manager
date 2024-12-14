use async_trait::async_trait;
use thiserror::Error;

use crate::model::{
    book::BookId,
    checkout::{
        event::{CreateCheckout, UpdateReturned},
        Checkout, CheckoutId,
    },
    user::UserId,
};

#[mockall::automock]
#[async_trait]
pub trait CheckoutRepository: Send + Sync {
    async fn create(&self, event: CreateCheckout) -> CheckoutRepositoryResult<()>;
    async fn find_unreturned_all(&self) -> CheckoutRepositoryResult<Vec<Checkout>>;
    async fn find_unreturned_by_user_id(
        &self,
        user_id: &UserId,
    ) -> CheckoutRepositoryResult<Vec<Checkout>>;
    async fn find_history_by_book_id(
        &self,
        book_id: &BookId,
    ) -> CheckoutRepositoryResult<Vec<Checkout>>;
    async fn update_returned(&self, event: UpdateReturned) -> CheckoutRepositoryResult<()>;
}

#[derive(Debug, Error)]
pub enum CheckoutRepositoryError {
    #[error("unexpected error occurred: {0}")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid saved entity: {0}")]
    InvalidSavedEntity(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("transaction error: {0}")]
    Transaction(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("book not found: {0}")]
    BookNotFound(BookId),

    #[error("book already checked out: {0}")]
    BookAlreadyCheckedOut(BookId),

    #[error("no resource was affected: {0}")]
    NoResourceAffected(String),

    #[error("cannot return: unable to process a return for a checkout (ID: {2}) of abook (ID: {0}) to a user (ID: {1}).")]
    CannotReturn(BookId, UserId, CheckoutId),
}

pub type CheckoutRepositoryResult<T> = Result<T, CheckoutRepositoryError>;
