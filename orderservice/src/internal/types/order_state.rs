use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use example_model::types::OrderItemResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderState {
    pub order_id: String,
    pub status: String,
    pub confirmed_items: Vec<OrderItemResult>,
    pub total_amount: f64,
    pub processed_at: DateTime<Utc>,
}
