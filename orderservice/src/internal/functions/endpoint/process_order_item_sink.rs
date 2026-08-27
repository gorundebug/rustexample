use std::sync::Arc;

use async_trait::async_trait;
use example_model::types::{OrderItem, OrderItemResult};
use inventory_service_api::processorderitem::{ProcessOrderItemRequest, ProcessOrderItemResponse};
use servicelib::{
    MessageContext, Payload,
    datasink::grpc::{EndpointHandler, HandlerResult, ResultContext, Sender, StreamContext},
    runtime::{
        config::GrpcEndpointConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};
use tokio::sync::Mutex;

use crate::internal::types::OrderState;

#[derive(Default)]
pub struct ProcessOrderItemSink;

#[derive(Default)]
pub struct ProcessOrderItemSinkState {
    order_id: String,
    item_id: String,
    sku: String,
    requested_qty: i32,
    unit_price: f64,
}

#[async_trait]
impl
    EndpointHandler<
        ProcessOrderItemSinkState,
        ProcessOrderItemRequest,
        ProcessOrderItemResponse,
        OrderItem,
        OrderItemResult,
        OrderState,
    > for ProcessOrderItemSink
{
    async fn begin_request(
        &self,
        context: MessageContext,
        _stream: StreamContext<OrderItem, OrderItemResult, OrderState>,
    ) -> HandlerResult<(MessageContext, ProcessOrderItemSinkState)> {
        Ok((context, ProcessOrderItemSinkState::default()))
    }

    async fn consume_message(
        &self,
        context: MessageContext,
        _stream: StreamContext<OrderItem, OrderItemResult, OrderState>,
        state: Arc<Mutex<ProcessOrderItemSinkState>>,
        value: Payload<OrderItem>,
        sender: &dyn Sender<ProcessOrderItemRequest>,
        _result_context: ResultContext,
    ) -> HandlerResult {
        let mut state = state.lock().await;
        state.order_id = value.order_id.clone();
        state.item_id = value.item_id.clone();
        state.sku = value.sku.clone();
        state.requested_qty = value.quantity;
        state.unit_price = value.unit_price;
        sender
            .send(
                context,
                ProcessOrderItemRequest {
                    order_id: value.order_id.clone(),
                    item_id: value.item_id.clone(),
                    sku: value.sku.clone(),
                    quantity: value.quantity,
                },
            )
            .await
    }

    async fn handle_response(
        &self,
        context: MessageContext,
        stream: StreamContext<OrderItem, OrderItemResult, OrderState>,
        state: Arc<Mutex<ProcessOrderItemSinkState>>,
        response: ProcessOrderItemResponse,
    ) -> HandlerResult {
        let state = state.lock().await;
        stream
            .collect(
                context,
                OrderItemResult {
                    order_id: state.order_id.clone(),
                    item_id: state.item_id.clone(),
                    sku: state.sku.clone(),
                    requested_qty: state.requested_qty,
                    available_qty: response.available_qty,
                    reserved: response.reserved,
                    status: response.status,
                    unit_price: state.unit_price,
                    error: String::new(),
                },
            )
            .await;
        Ok(())
    }

    async fn end_request(
        &self,
        context: MessageContext,
        stream: StreamContext<OrderItem, OrderItemResult, OrderState>,
        result: &HandlerResult,
        state: Arc<Mutex<ProcessOrderItemSinkState>>,
    ) {
        let Err(error) = result else {
            return;
        };
        let state = state.lock().await;
        stream
            .collect(
                context,
                OrderItemResult {
                    order_id: state.order_id.clone(),
                    item_id: state.item_id.clone(),
                    sku: state.sku.clone(),
                    requested_qty: state.requested_qty,
                    available_qty: 0,
                    reserved: false,
                    status: "PROCESSING_ERROR".to_owned(),
                    unit_price: state.unit_price,
                    error: error.to_string(),
                },
            )
            .await;
    }
}

pub fn make_process_order_item_sink(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &GrpcEndpointConfig,
) -> RuntimeResult<ProcessOrderItemSink> {
    Ok(ProcessOrderItemSink)
}
