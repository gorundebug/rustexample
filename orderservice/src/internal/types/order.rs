use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use example_model::types::OrderItem;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Order {
    pub id: String,
    pub customer_id: String,
    pub items: Vec<OrderItem>,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
    pub trace_id: String,
}
