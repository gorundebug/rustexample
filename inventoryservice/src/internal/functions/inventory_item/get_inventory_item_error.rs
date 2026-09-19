use async_trait::async_trait;
use example_model::types::OrderItemResult;
use serde::Deserialize;
use servicelib::{
    Collector, MessageContext,
    operators::map::MapFunction,
    runtime::{
        common::RuntimeStream,
        config::MapStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

#[derive(Deserialize)]
struct InventoryFailurePayload {
    order_id: String,
    item_id: String,
    sku: String,
    requested_qty: i32,
    available_qty: i32,
    unit_price: f64,
}

pub struct GetInventoryItemError;

#[async_trait]
impl MapFunction<String, OrderItemResult> for GetInventoryItemError {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &String,
        out: &Collector<OrderItemResult>,
    ) {
        let decoded = serde_json::from_str::<InventoryFailurePayload>(value);
        let result = match decoded {
            Ok(failure) => OrderItemResult {
                order_id: failure.order_id,
                item_id: failure.item_id,
                sku: failure.sku,
                requested_qty: failure.requested_qty,
                available_qty: failure.available_qty,
                reserved: false,
                status: "OUT_OF_STOCK".to_owned(),
                unit_price: failure.unit_price,
                error: "inventory is out of stock".to_owned(),
            },
            Err(error) => OrderItemResult {
                status: "PROCESSING_ERROR".to_owned(),
                error: format!("{error}: {value}"),
                ..Default::default()
            },
        };
        out.collect(context, result).await;
    }
}

pub async fn make_get_inventory_item_error(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &MapStreamConfig,
) -> RuntimeResult<GetInventoryItemError> {
    Ok(GetInventoryItemError)
}
