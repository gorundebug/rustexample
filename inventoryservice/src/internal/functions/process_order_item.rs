use std::sync::Arc;

use async_trait::async_trait;
use example_model::types::{OrderItem, OrderItemResult};
use inventory_service_api::processorderitem::{ProcessOrderItemRequest, ProcessOrderItemResponse};
use servicelib::{
    MessageContext, Payload,
    datasource::grpc::{EndpointHandler, HandlerResult, ResultContext, Sender, StreamContext},
    runtime::{
        config::GrpcEndpointConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct ProcessOrderItem;

#[async_trait]
impl
    EndpointHandler<
        (),
        ProcessOrderItemRequest,
        ProcessOrderItemResponse,
        OrderItem,
        OrderItemResult,
        OrderItemResult,
    > for ProcessOrderItem
{
    async fn begin_request(
        &self,
        context: MessageContext,
        _stream: StreamContext<OrderItem, OrderItemResult, OrderItemResult>,
    ) -> HandlerResult<(MessageContext, ())> {
        Ok((context, ()))
    }

    async fn consume_message(
        &self,
        context: MessageContext,
        stream: StreamContext<OrderItem, OrderItemResult, OrderItemResult>,
        _state: Arc<Mutex<()>>,
        request: ProcessOrderItemRequest,
        result_context: Arc<
            ResultContext<
                (),
                OrderItem,
                ProcessOrderItemResponse,
                OrderItemResult,
                OrderItemResult,
            >,
        >,
        _sender: Arc<dyn Sender<ProcessOrderItemResponse>>,
    ) -> HandlerResult<MessageContext> {
        let item_id = request.item_id.clone();
        result_context.set_result_callback(
            item_id,
            Arc::new(
                move |context, _stream, _state, value: Payload<OrderItemResult>, sender| {
                    Box::pin(async move {
                        sender
                            .send(
                                context,
                                ProcessOrderItemResponse {
                                    available_qty: value.available_qty,
                                    reserved: value.reserved,
                                    status: value.status.clone(),
                                },
                            )
                            .await
                            .is_ok()
                    })
                },
            ),
        );
        stream
            .collect(
                context.clone(),
                OrderItem {
                    order_id: request.order_id,
                    item_id: request.item_id,
                    sku: request.sku,
                    quantity: request.quantity,
                    unit_price: 0.0,
                },
            )
            .await;
        Ok(context)
    }

    async fn get_message_id(
        &self,
        _context: &MessageContext,
        _stream: &StreamContext<OrderItem, OrderItemResult, OrderItemResult>,
        _state: Arc<Mutex<()>>,
        value: &OrderItemResult,
    ) -> String {
        value.item_id.clone()
    }

    async fn eof(
        &self,
        _context: MessageContext,
        _stream: StreamContext<OrderItem, OrderItemResult, OrderItemResult>,
        _state: Arc<Mutex<()>>,
    ) {
    }

    async fn end_request(
        &self,
        _context: MessageContext,
        _stream: StreamContext<OrderItem, OrderItemResult, OrderItemResult>,
        result: &HandlerResult,
        _state: Arc<Mutex<()>>,
    ) -> HandlerResult {
        match result {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string().into()),
        }
    }
}

pub fn make_process_order_item(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &GrpcEndpointConfig,
) -> RuntimeResult<ProcessOrderItem> {
    Ok(ProcessOrderItem)
}
