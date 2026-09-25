use servicelib::{
    MessageContext,
    operators::FilterFunction,
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)]
pub struct CompleteCycleAnalytics;

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
) -> RuntimeResult<CompleteCycleAnalytics> {
    Ok(CompleteCycleAnalytics)
}
