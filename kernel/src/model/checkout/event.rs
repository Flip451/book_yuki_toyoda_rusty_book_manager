use chrono::{DateTime, Utc};

use crate::model::{book::BookId, user::UserId};

use super::CheckoutId;

pub struct CreateCheckout {
    pub book_id: BookId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
}

pub struct UpdateReturned {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub returned_by: UserId,
    pub returned_at: DateTime<Utc>,
}
