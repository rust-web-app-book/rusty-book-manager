use crate::model::id::{BookId, CheckoutId, UserId};
use chrono::{DateTime, Utc};
use derive_new::new;

#[derive(new)]
pub struct CreateCheckout {
    pub book_id: BookId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
}

#[derive(new)]
pub struct UpdateReturned {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub returned_by: UserId,
    pub returned_at: DateTime<Utc>,
}
