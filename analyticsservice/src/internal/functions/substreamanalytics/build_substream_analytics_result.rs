use servicelib::{
    Collector, MessageContext,
    operators::MapFunction,
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::{AnalyticsEvent, AnalyticsResult};

#[derive(Clone, Default)]
pub struct BuildSubstreamAnalyticsResult;

impl MapFunction<AnalyticsEvent, AnalyticsResult> for BuildSubstreamAnalyticsResult {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &AnalyticsEvent,
        out: &Collector<AnalyticsResult>,
    ) {
        out.collect(
            context,
            AnalyticsResult {
                key: value.key.clone(),
                total: value.value * 2,
                kind: "substream".into(),
            },
        )
        .await;
    }
}

pub async fn make_build_substream_analytics_result(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<BuildSubstreamAnalyticsResult> {
    Ok(BuildSubstreamAnalyticsResult)
}
