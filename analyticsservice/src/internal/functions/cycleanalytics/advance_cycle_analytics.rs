use async_trait::async_trait;
use servicelib::{
    Collector, MessageContext,
    operators::MapFunction,
    runtime::{
        common::RuntimeStream,
        config::MapStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)]
pub struct AdvanceCycleAnalytics;

#[async_trait]
impl MapFunction<AnalyticsEvent, AnalyticsEvent> for AdvanceCycleAnalytics {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &AnalyticsEvent,
        out: &Collector<AnalyticsEvent>,
    ) {
        let mut next = value.clone();
        next.value += 1;
        out.collect(context, next).await;
    }
}

pub async fn make_advance_cycle_analytics(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &MapStreamConfig,
) -> RuntimeResult<AdvanceCycleAnalytics> {
    Ok(AdvanceCycleAnalytics)
}
