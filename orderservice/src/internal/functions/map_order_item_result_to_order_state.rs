use async_trait::async_trait;
use chrono::Utc;
use example_model::types::OrderItemResult;
use servicelib::{
    Collector, MessageContext,
    operators::MapFunction,
    runtime::{
        common::RuntimeStream,
        config::MapStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::OrderState;

#[derive(Default)]
pub struct MapOrderItemResultToOrderState;

#[async_trait]
impl MapFunction<OrderItemResult, OrderState> for MapOrderItemResultToOrderState {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &OrderItemResult,
        out: &Collector<OrderState>,
    ) {
        let status = if value.reserved {
            "CONFIRMED"
        } else {
            "PARTIALLY_CONFIRMED"
        };
        out.collect(
            context,
            OrderState {
                order_id: value.order_id.clone(),
                status: status.to_owned(),
                confirmed_items: vec![value.clone()],
                total_amount: 0.0,
                processed_at: Utc::now(),
            },
        )
        .await;
    }
}

pub fn make_map_order_item_result_to_order_state(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &MapStreamConfig,
) -> RuntimeResult<MapOrderItemResultToOrderState> {
    Ok(MapOrderItemResultToOrderState)
}
