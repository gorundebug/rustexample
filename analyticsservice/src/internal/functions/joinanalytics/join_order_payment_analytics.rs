use servicelib::{
    Collector, MessageContext,
    operators::JoinFunction,
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::{AnalyticsEvent, AnalyticsResult};

#[derive(Clone, Default)]
pub struct JoinOrderPaymentAnalytics;

impl JoinFunction<String, AnalyticsEvent, AnalyticsEvent, AnalyticsResult>
    for JoinOrderPaymentAnalytics
{
    async fn join(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        key: String,
        left: Vec<AnalyticsEvent>,
        right: Vec<AnalyticsEvent>,
        out: &Collector<AnalyticsResult>,
    ) -> bool {
        let (Some(order), Some(payment)) = (left.first(), right.first()) else {
            return false;
        };
        out.collect(
            context,
            AnalyticsResult {
                key,
                total: order.value + payment.value,
                kind: "join".to_owned(),
            },
        )
        .await;
        true
    }
}

pub async fn make_join_order_payment_analytics(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<JoinOrderPaymentAnalytics> {
    Ok(JoinOrderPaymentAnalytics)
}
