use serde::{Deserialize, Serialize};

/// Inventory reservation result for a single order item. Fields: OrderID string, ItemID string, SKU string, RequestedQty int, AvailableQty int, Reserved bool, Status string (CONFIRMED / OUT_OF_STOCK / PROCESSING_ERROR), UnitPrice float64, Error string.
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
