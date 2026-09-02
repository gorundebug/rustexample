use async_trait::async_trait;
use example_model::types::OrderItem;
use servicelib::{
    Collector, MessageContext,
    operators::FlatMapFunction,
    runtime::{
        common::RuntimeStream,
        config::FlatMapStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::Order;

#[derive(Default)]
pub struct ProcessOrderItems;

#[async_trait]
impl FlatMapFunction<Order, OrderItem> for ProcessOrderItems {
    async fn flat_map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &Order,
        out: &Collector<OrderItem>,
    ) {
        for item in &value.items {
            let mut item = item.clone();
            item.order_id.clone_from(&value.id);
            out.collect(context.clone(), item).await;
        }
    }
}

pub async fn make_process_order_items(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &FlatMapStreamConfig,
) -> RuntimeResult<ProcessOrderItems> {
    Ok(ProcessOrderItems)
}
