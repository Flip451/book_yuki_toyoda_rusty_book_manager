use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use kernel::{model::book::BookIdError, repository::book::BookRepositoryError};
use registry::AppRegistry;
use thiserror::Error;
use uuid::Uuid;

use crate::model::book::{BookResponse, CreateBookRequest, CreateBookRequestError};

pub(crate) async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, BookHandlerError> {
    registry
        .book_repository()
        .create(req.try_into()?)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(BookHandlerError::from)
}

pub(crate) async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> Result<Json<Vec<BookResponse>>, BookHandlerError> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookResponse::from).collect())
        .map(Json)
        .map_err(BookHandlerError::from)
}

pub(crate) async fn show_book(
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
) -> Result<Json<BookResponse>, BookHandlerError> {
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

#[derive(Debug, Error)]
pub enum BookHandlerError {
    #[error("not found")]
    NotFound,

    #[error("repository error: {0}")]
    RepositoryError(#[from] BookRepositoryError),

    #[error("invalid create book request: {0}")]
    InvalidCreateBookRequest(#[from] CreateBookRequestError),

    #[error("invalid book id: {0}")]
    InvalidBookId(#[from] BookIdError),
}

impl IntoResponse for BookHandlerError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            BookHandlerError::NotFound => StatusCode::NOT_FOUND,
            e @ BookHandlerError::RepositoryError(_) => {
                tracing::error!(error.cause_chain = ?e, error = %e, "invalid entity");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            BookHandlerError::InvalidCreateBookRequest(_) => StatusCode::BAD_REQUEST,
            BookHandlerError::InvalidBookId(_) => StatusCode::BAD_REQUEST,
        };

        status_code.into_response()
    }
}
