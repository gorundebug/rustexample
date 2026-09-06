use std::sync::Arc;
use async_trait::async_trait;
use servicelib::{MessageContext, runtime::{common::{Consumer, Payload}, config::CustomEndpointConfig, datasource::StreamContext, environment::{RuntimeEnvironment, RuntimeResult}}, datasource::localsource::{DataProducer, EndpointHandler, HandlerError, HandlerResult, ResultContext}};
use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)] pub struct AnalyticsPaymentsSource;
#[async_trait]
impl DataProducer<AnalyticsEvent> for AnalyticsPaymentsSource {
    async fn start(&self, context: MessageContext, consumer: Arc<dyn Consumer<AnalyticsEvent>>) -> HandlerResult { for value in [AnalyticsEvent { key: "high-value".into(), value: 20, kind: "payment".into() }, AnalyticsEvent { key: "standard".into(), value: 2, kind: "payment".into() }] { consumer.consume(context.clone(), Payload::new(value)).await; } Ok(()) }
    async fn stop(&self, _context: MessageContext) {}
}
#[async_trait]
impl EndpointHandler<(), AnalyticsEvent, (), String> for AnalyticsPaymentsSource {
    fn concurrency(&self, _stream: &StreamContext<AnalyticsEvent, (), String>) -> usize { 0 }
    async fn begin_request(&self, context: MessageContext, _stream: StreamContext<AnalyticsEvent, (), String>) -> Result<(MessageContext, ()), HandlerError> { Ok((context, ())) }
    async fn consume_message(&self, context: MessageContext, stream: StreamContext<AnalyticsEvent, (), String>, _state: Arc<()>, value: Payload<AnalyticsEvent>, result: Arc<ResultContext<(), AnalyticsEvent, (), String>>) -> HandlerResult { stream.collect(context, value.into_value()).await; result.done(); Ok(()) }
    fn get_message_id(&self, _context: &MessageContext, _stream: &StreamContext<AnalyticsEvent, (), String>, _state: &(), _value: &()) -> String { String::new() }
    async fn end_request(&self, _context: MessageContext, _stream: StreamContext<AnalyticsEvent, (), String>, _result: &HandlerResult, _state: Arc<()>) {}
}
pub async fn make_analytics_payments_source(_context: MessageContext, _environment: RuntimeEnvironment, _config: &CustomEndpointConfig) -> RuntimeResult<AnalyticsPaymentsSource> { Ok(AnalyticsPaymentsSource) }
