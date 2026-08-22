use serde::{Deserialize, Serialize};

/// Final order-processing event consumed by the analytics service.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderProcessed {
    pub order_id: String,
    pub status: String,
    pub processed_at: String,
    pub total_items: usize,
    pub confirmed_items: usize,
    pub failure_reason: String,
}
