pub mod event;

use chrono::DateTime;
use chrono::Utc;
use derive_getters::Dissolve;
use uuid::Uuid;

use crate::impl_entity;
use crate::tuple_value_object_with_simple_error;

use super::book::Author;
use super::book::BookId;
use super::book::Isbn;
use super::book::Title;
use super::user::UserId;

tuple_value_object_with_simple_error!(CheckoutId, Uuid, CheckoutIdError);

#[derive(Debug, derive_new::new, Dissolve)]
pub struct Checkout {
    checkout_id: CheckoutId,
    checked_out_by: UserId,
    checked_out_at: DateTime<Utc>,
    returned_at: Option<DateTime<Utc>>,
    book: CheckoutBook,
}

impl_entity!(Checkout, checkout_id, CheckoutId);

#[derive(Debug, derive_new::new, Dissolve)]
pub struct CheckoutBook {
    book_id: BookId,
    title: Title,
    author: Author,
    isbn: Isbn,
}
