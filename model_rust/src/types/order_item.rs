use serde::{Deserialize, Serialize};

/// A single line item within an order. Fields: OrderID string, ItemID string, SKU string, Quantity int.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderItem {
    pub order_id: String,
    pub item_id: String,
    pub sku: String,
    pub quantity: i32,
    pub unit_price: f64,
}
