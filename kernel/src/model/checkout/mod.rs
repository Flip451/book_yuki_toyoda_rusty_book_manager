pub mod event;

use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use crate::impl_entity;
use crate::tuple_value_object_with_simple_error;

use super::book::Author;
use super::book::BookId;
use super::book::Isbn;
use super::book::Title;
use super::user::UserId;

tuple_value_object_with_simple_error!(CheckoutId, Uuid, CheckoutIdError);

pub struct Checkout {
    pub checkout_id: CheckoutId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBook,
}

impl_entity!(Checkout, checkout_id, CheckoutId);

pub struct CheckoutBook {
    pub book_id: BookId,
    pub title: Title,
    pub author: Author,
    pub isbn: Isbn,
}
