use example_model::types::OrderProcessed;
use servicelib::{
    Collector, MessageContext,
    operators::MapFunction,
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::OrderState;

#[derive(Default)]
pub struct MapToOrderProcessed;

impl MapFunction<OrderState, OrderProcessed> for MapToOrderProcessed {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &OrderState,
        out: &Collector<OrderProcessed>,
    ) {
        let confirmed_items = value
            .confirmed_items
            .iter()
            .filter(|item| item.reserved)
            .count();
        out.collect(
            context,
            OrderProcessed {
                order_id: value.order_id.clone(),
                status: value.status.clone(),
                processed_at: value.processed_at.to_rfc3339(),
                total_items: value.confirmed_items.len(),
                confirmed_items,
                failure_reason: if value.status == "CONFIRMED" {
                    String::new()
                } else {
                    value.status.clone()
                },
            },
        )
        .await;
    }
}

pub async fn make_map_to_order_processed(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<MapToOrderProcessed> {
    Ok(MapToOrderProcessed)
}
