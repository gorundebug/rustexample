use crate::internal::types::{AnalyticsEvent, AnalyticsResult};
use servicelib::{
    MessageContext,
    operators::{MultiJoinFunction, downcast_join_values},
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
        store::JoinValues,
    },
};

#[derive(Clone, Default)]
pub struct MultiJoinAnalyticsEvents;
impl MultiJoinFunction<String, AnalyticsResult> for MultiJoinAnalyticsEvents {
    async fn multi_join(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        key: String,
        values: JoinValues,
        out: &impl servicelib::runtime::collector::Collect<AnalyticsResult>,
    ) -> bool {
        if values.len() != 3 {
            return false;
        }
        let orders = downcast_join_values::<AnalyticsEvent>(&values, 0);
        let payments = downcast_join_values::<AnalyticsEvent>(&values, 1);
        let shipments = downcast_join_values::<AnalyticsEvent>(&values, 2);
        let (Some(order), Some(payment), Some(shipment)) =
            (orders.first(), payments.first(), shipments.first())
        else {
            return false;
        };
        out.collect(
            context,
            AnalyticsResult {
                key,
                total: order.value + payment.value + shipment.value,
                kind: "multi".to_owned(),
            },
        )
        .await;
        true
    }
}
pub async fn make_multi_join_analytics_events(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<MultiJoinAnalyticsEvents> {
    Ok(MultiJoinAnalyticsEvents)
}
