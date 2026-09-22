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

use crate::internal::types::AnalyticsResult;

#[derive(Clone, Default)]
pub struct SubstreamAnalyticsResultSink;

#[async_trait]
impl EndpointHandler<(), AnalyticsResult, String> for SubstreamAnalyticsResultSink {
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
        let value = value.into_value();
        if value.key == "substream" && value.total == 14 && value.kind == "substream" {
            Ok(())
        } else {
            Err(std::io::Error::other("unexpected substream analytics result").into())
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

pub async fn make_substream_analytics_result_sink(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<SubstreamAnalyticsResultSink> {
    Ok(SubstreamAnalyticsResultSink)
}
