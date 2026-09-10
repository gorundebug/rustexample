use async_trait::async_trait;
use servicelib::{
    MessageContext,
    operators::FilterFunction,
    runtime::{
        common::RuntimeStream,
        config::FilterStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)]
pub struct CompleteCycleAnalytics;

#[async_trait]
impl FilterFunction<AnalyticsEvent> for CompleteCycleAnalytics {
    async fn filter(
        &self,
        _context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &AnalyticsEvent,
    ) -> bool {
        value.value >= 3
    }
}

pub async fn make_complete_cycle_analytics(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &FilterStreamConfig,
) -> RuntimeResult<CompleteCycleAnalytics> {
    Ok(CompleteCycleAnalytics)
}
