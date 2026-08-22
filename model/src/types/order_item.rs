use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderItem {
    pub order_id: String,
    pub item_id: String,
    pub sku: String,
    pub quantity: i32,
    pub unit_price: f64,
}
