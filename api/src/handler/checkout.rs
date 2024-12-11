use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use kernel::{
    model::{
        book::BookIdError,
        checkout::{
            event::{CreateCheckout, UpdateReturned},
            CheckoutIdError,
        },
    },
    repository::checkout::CheckoutRepositoryError,
};
use registry::AppRegistry;
use uuid::Uuid;

use crate::{extractor::AuthorizedUser, model::checkout::CheckoutsResponse};

pub(crate) async fn checkout_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
) -> Result<StatusCode, CheckoutHandlerError> {
    let create_checkout = CreateCheckout {
        book_id: book_id.try_into()?,
        checked_out_by: user.user_id().clone(),
        checked_out_at: Utc::now(),
    };

    registry
        .checkout_repository()
        .create(create_checkout)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(CheckoutHandlerError::from)
}

pub(crate) async fn return_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path((book_id, checkout_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, CheckoutHandlerError> {
    let update_returned = UpdateReturned {
        book_id: book_id.try_into()?,
        checkout_id: checkout_id.try_into()?,
        returned_by: user.user_id().clone(),
        returned_at: Utc::now(),
    };

    registry
        .checkout_repository()
        .update_returned(update_returned)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(CheckoutHandlerError::from)
}

pub(crate) async fn checkout_history(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
) -> Result<Json<CheckoutsResponse>, CheckoutHandlerError> {
    let book_id = book_id.try_into()?;
    let checkout_history = registry
        .checkout_repository()
        .find_history_by_book_id(&book_id)
        .await?
        .into();
    Ok(Json(checkout_history))
}

pub(crate) async fn show_checked_out_list(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> Result<Json<CheckoutsResponse>, CheckoutHandlerError> {
    let checkout_history = registry
        .checkout_repository()
        .find_unreturned_all()
        .await?
        .into();
    Ok(Json(checkout_history))
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CheckoutHandlerError {
    #[error("invalid book id: {0}")]
    InvalidBookId(#[from] BookIdError),

    #[error("invalid checkout id: {0}")]
    InvalidCheckoutId(#[from] CheckoutIdError),

    #[error("checkout repository error: {0}")]
    CheckoutRepositoryError(#[from] CheckoutRepositoryError),
}

impl IntoResponse for CheckoutHandlerError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
