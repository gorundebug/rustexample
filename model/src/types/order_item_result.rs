use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderItemResult {
    pub order_id: String,
    pub item_id: String,
    pub sku: String,
    pub requested_qty: i32,
    pub available_qty: i32,
    pub reserved: bool,
    pub status: String,
    pub unit_price: f64,
    pub error: String,
}
