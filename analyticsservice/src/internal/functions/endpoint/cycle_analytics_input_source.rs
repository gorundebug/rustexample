use std::sync::Arc;

use async_trait::async_trait;
use servicelib::{
    MessageContext,
    datasource::localsource::{
        DataProducer, EndpointHandler, HandlerError, HandlerResult, ResultContext,
    },
    runtime::{
        common::{Consumer, Payload},
        config::CustomEndpointConfig,
        datasource::StreamContext,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::AnalyticsEvent;

#[derive(Clone, Default)]
pub struct CycleAnalyticsInputSource;

#[async_trait]
impl DataProducer<AnalyticsEvent> for CycleAnalyticsInputSource {
    async fn start(
        &self,
        context: MessageContext,
        consumer: Arc<dyn Consumer<AnalyticsEvent>>,
    ) -> HandlerResult {
        consumer
            .consume(
                context,
                Payload::new(AnalyticsEvent {
                    key: "cycle".into(),
                    value: 0,
                    kind: "cycle".into(),
                }),
            )
            .await;
        Ok(())
    }

    async fn stop(&self, _context: MessageContext) {}
}

#[async_trait]
impl EndpointHandler<(), AnalyticsEvent, (), String> for CycleAnalyticsInputSource {
    fn concurrency(&self, _stream: &StreamContext<AnalyticsEvent, (), String>) -> usize {
        0
    }

    async fn begin_request(
        &self,
        context: MessageContext,
        _stream: StreamContext<AnalyticsEvent, (), String>,
    ) -> Result<(MessageContext, ()), HandlerError> {
        Ok((context, ()))
    }

    async fn consume_message(
        &self,
        context: MessageContext,
        stream: StreamContext<AnalyticsEvent, (), String>,
        _state: Arc<()>,
        value: Payload<AnalyticsEvent>,
        result: Arc<ResultContext<(), AnalyticsEvent, (), String>>,
    ) -> HandlerResult {
        stream.collect(context, value.into_value()).await;
        result.done();
        Ok(())
    }

    fn get_message_id(
        &self,
        _context: &MessageContext,
        _stream: &StreamContext<AnalyticsEvent, (), String>,
        _state: &(),
        _value: &(),
    ) -> String {
        String::new()
    }

    async fn end_request(
        &self,
        _context: MessageContext,
        _stream: StreamContext<AnalyticsEvent, (), String>,
        _result: &HandlerResult,
        _state: Arc<()>,
    ) {
    }
}

pub async fn make_cycle_analytics_input_source(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &CustomEndpointConfig,
) -> RuntimeResult<CycleAnalyticsInputSource> {
    Ok(CycleAnalyticsInputSource)
}
