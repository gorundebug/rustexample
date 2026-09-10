use async_trait::async_trait;
use servicelib::{
    MessageContext,
    datasink::localsink::{EndpointHandler, HandlerResult},
    runtime::{
        common::{Payload, RuntimeStream},
        config::CustomEndpointConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
        stream::Stream,
    },
};

use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)]
pub struct CycleAnalyticsResultSink;

#[async_trait]
impl EndpointHandler<(), AnalyticsEvent, String> for CycleAnalyticsResultSink {
    fn get_stream_id(&self, _context: &MessageContext, value: &AnalyticsEvent) -> String {
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
        value: Payload<AnalyticsEvent>,
        _result_stream: &Stream<String>,
    ) -> HandlerResult {
        let value = value.into_value();
        if value.key == "cycle" && value.value == 3 && value.kind == "cycle" {
            Ok(())
        } else {
            Err(std::io::Error::other("unexpected cycle analytics result").into())
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

pub async fn make_cycle_analytics_result_sink(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &CustomEndpointConfig,
) -> RuntimeResult<CycleAnalyticsResultSink> {
    Ok(CycleAnalyticsResultSink)
}
