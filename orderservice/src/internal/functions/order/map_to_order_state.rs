use async_trait::async_trait;
use chrono::Utc;
use servicelib::{
    Collector, MessageContext,
    operators::MapFunction,
    runtime::{
        common::RuntimeStream,
        config::MapStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::{Order, OrderState};

#[derive(Default)]
pub struct MapToOrderState;

#[async_trait]
impl MapFunction<Order, OrderState> for MapToOrderState {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &Order,
        out: &Collector<OrderState>,
    ) {
        out.collect(
            context,
            OrderState {
                order_id: value.id.clone(),
                status: "TIMED_OUT".to_owned(),
                confirmed_items: Vec::new(),
                total_amount: value.total_amount,
                processed_at: Utc::now(),
            },
        )
        .await;
    }
}

pub async fn make_map_to_order_state(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &MapStreamConfig,
) -> RuntimeResult<MapToOrderState> {
    Ok(MapToOrderState)
}
