use chrono::{DateTime, Utc};
use kernel::model::{
    checkout::{Checkout, CheckoutBook},
    value_object::ValueObject,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutsResponse {
    pub items: Vec<CheckoutResponse>,
}

impl From<Vec<Checkout>> for CheckoutsResponse {
    fn from(checkouts: Vec<Checkout>) -> Self {
        CheckoutsResponse {
            items: checkouts.into_iter().map(CheckoutResponse::from).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutResponse {
    pub id: Uuid,
    pub checked_out_by: Uuid,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBookResponse,
}

impl From<Checkout> for CheckoutResponse {
    fn from(checkout: Checkout) -> Self {
        let (checkout_id, checked_out_by, checked_out_at, returned_at, book) = checkout.dissolve();
        CheckoutResponse {
            id: checkout_id.into_inner(),
            checked_out_by: checked_out_by.into_inner(),
            checked_out_at,
            returned_at,
            book: book.into(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutBookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<CheckoutBook> for CheckoutBookResponse {
    fn from(book: CheckoutBook) -> Self {
        let (book_id, title, author, isbn) = book.dissolve();
        CheckoutBookResponse {
            id: book_id.into_inner(),
            title: title.into_inner(),
            author: author.into_inner(),
            isbn: isbn.into_inner(),
        }
    }
}
