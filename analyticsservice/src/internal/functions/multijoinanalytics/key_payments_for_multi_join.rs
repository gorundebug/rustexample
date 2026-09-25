use crate::internal::types::AnalyticsEvent;
use servicelib::{
    MessageContext,
    operators::KeyByFunction,
    runtime::{
        common::RuntimeStream,
        datastruct::KeyValue,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

#[derive(Clone, Default)]
pub struct KeyPaymentsForMultiJoin;
impl KeyByFunction<AnalyticsEvent, String, AnalyticsEvent> for KeyPaymentsForMultiJoin {
    async fn key_by(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &AnalyticsEvent,
        out: &impl servicelib::runtime::collector::Collect<KeyValue<String, AnalyticsEvent>>,
    ) {
        out.collect(
            context,
            KeyValue {
                key: value.key.clone(),
                value: value.clone(),
            },
        )
        .await;
    }
}
pub async fn make_key_payments_for_multi_join(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<KeyPaymentsForMultiJoin> {
    Ok(KeyPaymentsForMultiJoin)
}
