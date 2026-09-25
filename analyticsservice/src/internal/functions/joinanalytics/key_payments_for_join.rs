use servicelib::{
    MessageContext,
    operators::KeyByFunction,
    runtime::{
        common::RuntimeStream,
        datastruct::KeyValue,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)]
pub struct KeyPaymentsForJoin;

impl KeyByFunction<AnalyticsEvent, String, AnalyticsEvent> for KeyPaymentsForJoin {
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

pub async fn make_key_payments_for_join(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<KeyPaymentsForJoin> {
    Ok(KeyPaymentsForJoin)
}
