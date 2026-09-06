use async_trait::async_trait;
use servicelib::{MessageContext, runtime::{common::{Payload, RuntimeStream}, config::CustomEndpointConfig, environment::{RuntimeEnvironment, RuntimeResult}, stream::Stream}, datasink::localsink::{EndpointHandler, HandlerResult}};
use crate::internal::types::AnalyticsResult;

#[derive(Clone, Default)] pub struct HighValueAnalyticsSink;
#[async_trait]
impl EndpointHandler<(), AnalyticsResult, String> for HighValueAnalyticsSink {
    fn get_stream_id(&self, _context: &MessageContext, value: &AnalyticsResult) -> String { value.key.clone() }
    async fn begin_request(&self, context: MessageContext, _stream: &dyn RuntimeStream) -> (MessageContext, ()) { (context, ()) }
    async fn consume_message(&self, _context: MessageContext, _stream: &dyn RuntimeStream, _state: &mut (), value: Payload<AnalyticsResult>, _result_stream: &Stream<String>) -> HandlerResult { if value.key == "high-value" && value.total == 60 && value.kind == "multi" { Ok(()) } else { Err(std::io::Error::other("unexpected high-value analytics result").into()) } }
    async fn end_request(&self, _context: MessageContext, _stream: &dyn RuntimeStream, _result: &HandlerResult, _state: ()) {}
}
pub async fn make_high_value_analytics_sink(_context: MessageContext, _environment: RuntimeEnvironment, _config: &CustomEndpointConfig) -> RuntimeResult<HighValueAnalyticsSink> { Ok(HighValueAnalyticsSink) }
