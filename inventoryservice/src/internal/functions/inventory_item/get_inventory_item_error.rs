use crate::internal::types::InventoryFailure;
use example_model::types::OrderItemResult;
use servicelib::{
    MessageContext,
    operators::map::MapFunction,
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};
pub struct GetInventoryItemError;

impl MapFunction<InventoryFailure, OrderItemResult> for GetInventoryItemError {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &InventoryFailure,
        out: &impl servicelib::runtime::collector::Collect<OrderItemResult>,
    ) {
        let item = &value.item;
        out.collect(context, OrderItemResult {
            order_id: item.order_id.clone(),
            item_id: item.item_id.clone(),
            sku: item.sku.clone(),
            requested_qty: item.quantity,
            available_qty: value.available_qty,
            reserved: false,
            status: "OUT_OF_STOCK".to_owned(),
            unit_price: item.unit_price,
            error: "inventory is out of stock".to_owned(),
        }).await;
    }
}

pub async fn make_get_inventory_item_error(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<GetInventoryItemError> {
    Ok(GetInventoryItemError)
}
