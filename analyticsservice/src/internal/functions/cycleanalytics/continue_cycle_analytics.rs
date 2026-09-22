use async_trait::async_trait;
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
pub struct ContinueCycleAnalytics;

#[async_trait]
impl FilterFunction<AnalyticsEvent> for ContinueCycleAnalytics {
    async fn filter(
        &self,
        _context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &AnalyticsEvent,
    ) -> bool {
        value.value < 3
    }
}

pub async fn make_continue_cycle_analytics(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<ContinueCycleAnalytics> {
    Ok(ContinueCycleAnalytics)
}
