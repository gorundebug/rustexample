use async_trait::async_trait;
use example_model::types::OrderProcessed;
use servicelib::{
    MessageContext, Payload,
    datasink::kafka::{EndpointHandler, HandlerResult, SinkMessage, StreamContext},
    runtime::{
        config::KafkaEndpointConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

#[derive(Default)]
pub struct OrderProcessedEndpointSink;

#[async_trait]
impl EndpointHandler<(), OrderProcessed, String, String> for OrderProcessedEndpointSink {
    fn get_stream_id(&self, _context: &MessageContext, value: &OrderProcessed) -> String {
        value.order_id.clone()
    }

    async fn begin_request(
        &self,
        context: MessageContext,
        _stream: StreamContext<OrderProcessed, String, String>,
    ) -> (MessageContext, ()) {
        (context, ())
    }

    async fn consume_message(
        &self,
        _context: MessageContext,
        _stream: StreamContext<OrderProcessed, String, String>,
        _state: &mut (),
        value: Payload<OrderProcessed>,
        message: &mut SinkMessage<String, String>,
    ) -> HandlerResult {
        message.key = Some(value.order_id.as_bytes().to_vec());
        message.value = serde_json::to_vec(&*value)?;
        message.send(|_partition, _offset, error| {
            error.map_or_else(String::new, |error| error.to_string())
        });
        Ok(())
    }

    async fn end_request(
        &self,
        _context: MessageContext,
        _stream: StreamContext<OrderProcessed, String, String>,
        _result: &HandlerResult,
        _state: (),
    ) {
    }
}

pub fn make_order_processed_endpoint_sink(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &KafkaEndpointConfig,
) -> RuntimeResult<OrderProcessedEndpointSink> {
    Ok(OrderProcessedEndpointSink)
}
