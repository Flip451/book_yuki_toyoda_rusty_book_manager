use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use garde::Validate;
use kernel::{
    model::book::{event::DeleteBook, BookIdError},
    repository::book::BookRepositoryError,
};
use registry::AppRegistry;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    extractor::AuthorizedUser,
    model::book::{
        BookListQuery, BookResponse, CreateBookRequest, CreateBookRequestError,
        PaginatedBookResponse, UpdateBookRequest, UpdateBookRequestError, UpdateBookRequestWithIds,
    },
};

pub(crate) async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, BookHandlerError> {
    req.validate()?;

    registry
        .book_repository()
        .create(req.try_into()?, user.user_id().clone())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(BookHandlerError::from)
}

pub(crate) async fn show_book_list(
    _user: AuthorizedUser,
    Query(req): Query<BookListQuery>,
    State(registry): State<AppRegistry>,
) -> Result<Json<PaginatedBookResponse>, BookHandlerError> {
    req.validate()?;

    registry
        .book_repository()
        .find_all(req.into())
        .await
        .map(PaginatedBookResponse::from)
        .map(Json)
        .map_err(BookHandlerError::from)
}

#[tracing::instrument(
    skip(_user, registry),
    fields(
        user_id = %_user.user_id(),
    )
)]
pub(crate) async fn show_book(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
) -> Result<Json<BookResponse>, BookHandlerError> {
    tracing::info!("show book called");

    let res = registry
        .book_repository()
        .find_by_id(&book_id.try_into()?)
        .await
        .map_err(BookHandlerError::from)?;
    match res {
        Some(book) => Ok(Json(book.into())),
        None => Err(BookHandlerError::NotFound),
    }
}

pub(crate) async fn update_book(
    user: AuthorizedUser,
    Path(book_id): Path<Uuid>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> Result<StatusCode, BookHandlerError> {
    req.validate()?;

    registry
        .book_repository()
        .update(
            UpdateBookRequestWithIds::new(book_id.try_into()?, user.user_id().clone(), req)
                .try_into()?,
        )
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(BookHandlerError::from)
}

pub(crate) async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<Uuid>,
    State(registry): State<AppRegistry>,
) -> Result<StatusCode, BookHandlerError> {
    let delete_book = DeleteBook {
        book_id: book_id.try_into()?,
        requested_by: user.user_id().clone(),
    };

    registry
        .book_repository()
        .delete(delete_book)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(BookHandlerError::from)
}

#[derive(Debug, Error)]
pub enum BookHandlerError {
    #[error("validation error: {0}")]
    ValidationError(#[from] garde::Report),

    #[error("not found")]
    NotFound,

    #[error("repository error: {0}")]
    RepositoryError(#[from] BookRepositoryError),

    #[error("invalid create book request: {0}")]
    InvalidCreateBookRequest(#[from] CreateBookRequestError),

    #[error("invalid update book request: {0}")]
    InvalidUpdateBookRequest(#[from] UpdateBookRequestError),

    #[error("invalid book id: {0}")]
    InvalidBookId(#[from] BookIdError),
}

impl IntoResponse for BookHandlerError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            BookHandlerError::ValidationError(_) => StatusCode::BAD_REQUEST,
            BookHandlerError::NotFound => StatusCode::NOT_FOUND,
            e @ BookHandlerError::RepositoryError(_) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
            BookHandlerError::InvalidCreateBookRequest(_) => StatusCode::BAD_REQUEST,
            BookHandlerError::InvalidBookId(_) => StatusCode::BAD_REQUEST,
            BookHandlerError::InvalidUpdateBookRequest(_) => StatusCode::BAD_REQUEST,
        };

        status_code.into_response()
    }
}
