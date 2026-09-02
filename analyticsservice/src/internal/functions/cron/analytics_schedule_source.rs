use async_trait::async_trait;
use servicelib::{
    Collector, MessageContext, ScheduleEndpointFunction, ScheduleTrigger,
    runtime::{
        config::CronEndpointConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

#[derive(Clone, Default)]
pub struct AnalyticsScheduleSource;

#[async_trait]
impl ScheduleEndpointFunction<String> for AnalyticsScheduleSource {
    async fn on_trigger(
        &self,
        context: MessageContext,
        trigger: ScheduleTrigger,
        out: &Collector<String>,
    ) {
        out.collect(context, trigger.trigger_id).await;
    }
}

pub async fn make_analytics_schedule_source(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &CronEndpointConfig,
) -> RuntimeResult<AnalyticsScheduleSource> {
    Ok(AnalyticsScheduleSource)
}
