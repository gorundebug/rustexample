use std::sync::Arc;

use async_trait::async_trait;
use example_model::types::OrderProcessed;
use servicelib::{
    MessageContext,
    datasource::kafka::{
        ConsumerMessage, EndpointHandler, HandlerError, HandlerResult, ResultCallback,
        ResultContext, StreamContext,
    },
    runtime::{
        config::KafkaEndpointConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct OrderProcessedEndpointSource;

#[async_trait]
impl EndpointHandler<(), OrderProcessed, OrderProcessed, String> for OrderProcessedEndpointSource {
    fn concurrency(
        &self,
        _stream: &StreamContext<OrderProcessed, OrderProcessed, String>,
    ) -> usize {
        0
    }

    async fn begin_request(
        &self,
        context: MessageContext,
        _stream: StreamContext<OrderProcessed, OrderProcessed, String>,
    ) -> Result<(MessageContext, ()), HandlerError> {
        Ok((context, ()))
    }

    async fn consume_message(
        &self,
        context: MessageContext,
        stream: StreamContext<OrderProcessed, OrderProcessed, String>,
        _state: Arc<Mutex<()>>,
        message: ConsumerMessage,
        result: Arc<ResultContext<(), OrderProcessed, OrderProcessed, String>>,
    ) -> HandlerResult {
        let value: OrderProcessed = serde_json::from_slice(&message.value)?;
        let message_id = value.order_id.clone();
        let done = Arc::clone(&result);
        let callback: ResultCallback<(), OrderProcessed, OrderProcessed, String> =
            Arc::new(move |_context, _stream, _state, _value| {
                let message = message.clone();
                let done = Arc::clone(&done);
                Box::pin(async move {
                    message.mark_message("processed");
                    done.done();
                    true
                })
            });
        result.set_result_callback(message_id, callback);
        stream.collect(context, value).await;
        Ok(())
    }

    async fn get_message_id(
        &self,
        _context: &MessageContext,
        _stream: &StreamContext<OrderProcessed, OrderProcessed, String>,
        _state: Arc<Mutex<()>>,
        value: &OrderProcessed,
    ) -> String {
        value.order_id.clone()
    }

    async fn end_request(
        &self,
        _context: MessageContext,
        _stream: StreamContext<OrderProcessed, OrderProcessed, String>,
        _result: &HandlerResult,
        _state: Arc<Mutex<()>>,
    ) {
    }
}

pub fn make_order_processed_endpoint_source(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &KafkaEndpointConfig,
) -> RuntimeResult<OrderProcessedEndpointSource> {
    Ok(OrderProcessedEndpointSource)
}
