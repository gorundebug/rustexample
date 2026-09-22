use crate::internal::types::AnalyticsResult;
use async_trait::async_trait;
use servicelib::{
    MessageContext,
    datasink::localsink::{EndpointHandler, HandlerResult},
    runtime::{
        common::{Payload, RuntimeStream},
        environment::{RuntimeEnvironment, RuntimeResult},
        stream::Stream,
    },
};

#[derive(Clone, Default)]
pub struct JoinedAnalyticsSink;
#[async_trait]
impl EndpointHandler<(), AnalyticsResult, String> for JoinedAnalyticsSink {
    fn get_stream_id(&self, _context: &MessageContext, value: &AnalyticsResult) -> String {
        value.key.clone()
    }
    async fn begin_request(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
    ) -> (MessageContext, ()) {
        (context, ())
    }
    async fn consume_message(
        &self,
        _context: MessageContext,
        _stream: &dyn RuntimeStream,
        _state: &mut (),
        value: Payload<AnalyticsResult>,
        _result_stream: &Stream<String>,
    ) -> HandlerResult {
        let expected = match value.key.as_str() {
            "high-value" => Some(30),
            "standard" => Some(3),
            _ => None,
        };
        if expected == Some(value.total) && value.kind == "join" {
            Ok(())
        } else {
            Err(std::io::Error::other("unexpected joined analytics result").into())
        }
    }
    async fn end_request(
        &self,
        _context: MessageContext,
        _stream: &dyn RuntimeStream,
        _result: &HandlerResult,
        _state: (),
    ) {
    }
}
pub async fn make_joined_analytics_sink(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<JoinedAnalyticsSink> {
    Ok(JoinedAnalyticsSink)
}
