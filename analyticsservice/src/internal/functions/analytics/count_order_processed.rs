use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use example_model::types::OrderProcessed;
use servicelib::{
    Collector, MessageContext,
    operators::ProcessFunction,
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

#[derive(Default)]
pub struct CountOrderProcessed {
    counters: Arc<Counters>,
}

#[derive(Default)]
struct Counters {
    successful: AtomicU64,
    unsuccessful: AtomicU64,
}

impl ProcessFunction<OrderProcessed, OrderProcessed, String> for CountOrderProcessed {
    async fn process(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &OrderProcessed,
        out: &Collector<OrderProcessed>,
        _error: &Collector<String>,
    ) {
        let counter = if value.status == "CONFIRMED" {
            &self.counters.successful
        } else {
            &self.counters.unsuccessful
        };
        counter.fetch_add(1, Ordering::Relaxed);
        out.collect(context, value.clone()).await;
    }
}

pub async fn make_count_order_processed(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<CountOrderProcessed> {
    Ok(CountOrderProcessed::default())
}
