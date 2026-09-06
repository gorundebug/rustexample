use async_trait::async_trait;
use servicelib::{Collector, MessageContext, operators::KeyByFunction, runtime::{common::RuntimeStream, config::KeyByStreamConfig, datastruct::KeyValue, environment::{RuntimeEnvironment, RuntimeResult}}};
use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)] pub struct KeyShipmentsForMultiJoin;
#[async_trait]
impl KeyByFunction<AnalyticsEvent, String, AnalyticsEvent> for KeyShipmentsForMultiJoin {
    async fn key_by(&self, context: MessageContext, _stream: &dyn RuntimeStream, value: &AnalyticsEvent, out: &Collector<KeyValue<String, AnalyticsEvent>>) { out.collect(context, KeyValue { key: value.key.clone(), value: value.clone() }).await; }
}
pub async fn make_key_shipments_for_multi_join(_context: MessageContext, _environment: RuntimeEnvironment, _config: &KeyByStreamConfig) -> RuntimeResult<KeyShipmentsForMultiJoin> { Ok(KeyShipmentsForMultiJoin) }
