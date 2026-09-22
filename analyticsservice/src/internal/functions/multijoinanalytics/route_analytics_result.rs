use crate::internal::types::AnalyticsResult;
use servicelib::{
    MessageContext,
    operators::BuildSwitchFunction,
    runtime::{
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

#[derive(Clone, Default)]
pub struct RouteAnalyticsResult;
impl BuildSwitchFunction<AnalyticsResult> for RouteAnalyticsResult {
    fn select(&self, value: &AnalyticsResult) -> usize {
        if value.total >= 50 { 0 } else { 1 }
    }
}
pub async fn make_route_analytics_result(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<RouteAnalyticsResult> {
    Ok(RouteAnalyticsResult)
}
