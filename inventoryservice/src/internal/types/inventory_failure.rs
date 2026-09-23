use example_model::types::OrderItem;
use serde::{Deserialize, Serialize};

/// Inventory shortage data carried by the business error branch.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InventoryFailure {
    pub item: OrderItem,
    pub available_qty: i32,
}
