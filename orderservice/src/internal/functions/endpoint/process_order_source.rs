use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use async_trait::async_trait;
use axum::http::{HeaderValue, StatusCode, header::CONTENT_TYPE};
use chrono::Utc;
use example_model::types::{OrderItem, OrderItemResult};
use order_service_api::models::{
    ProcessOrderRequest, ProcessOrderResponse, ProcessOrderResponseItem,
};
use servicelib::{
    MessageContext, Payload,
    datasource::http::{
        EndpointHandler, HandlerData, HandlerError, HandlerResult, ResultCallback, ResultContext,
    },
    runtime::{
        config::HttpEndpointConfig,
        datasource::StreamContext,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::internal::types::{Order, OrderState};

#[derive(Clone)]
pub struct ProcessOrderSource {
    timeout_ms: Arc<AtomicU64>,
}

pub struct ProcessOrderSourceState {
    order: Order,
    expected_items: usize,
    results: Vec<OrderItemResult>,
    response_sent: bool,
}

impl ProcessOrderSource {
    pub fn set_timeout_ms(&self, timeout_ms: u64) {
        self.timeout_ms.store(timeout_ms, Ordering::Release);
    }

    fn bad_request(data: &HandlerData, message: &str) -> HandlerError {
        data.set_status(StatusCode::BAD_REQUEST);
        data.set_header(
            CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        data.set_response_body(format!("{message}\n").into_bytes());
        message.to_owned().into()
    }
}

#[async_trait]
impl
    EndpointHandler<
        ProcessOrderSourceState,
        ProcessOrderRequest,
        ProcessOrderResponse,
        Order,
        OrderState,
        String,
    > for ProcessOrderSource
{
    fn reload(&self, _config: &HttpEndpointConfig, default_timeout_ms: u64) {
        self.set_timeout_ms(default_timeout_ms);
    }

    async fn begin_request(
        &self,
        context: MessageContext,
        _stream: StreamContext<Order, OrderState, String>,
        data: HandlerData,
    ) -> Result<(MessageContext, ProcessOrderSourceState), HandlerError> {
        let body: ProcessOrderRequest = serde_json::from_slice(&data.body)
            .map_err(|error| Self::bad_request(&data, &format!("invalid JSON body: {error}")))?;
        if body.items.is_empty() {
            return Err(Self::bad_request(&data, "items must not be empty"));
        }
        if body.items.iter().any(|item| item.quantity <= 0) {
            return Err(Self::bad_request(&data, "all quantities must be positive"));
        }

        let order_id = data
            .headers
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let trace_id = data
            .headers
            .get("x-trace")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_owned();
        let items: Vec<_> = body
            .items
            .iter()
            .map(|item| OrderItem {
                order_id: order_id.clone(),
                item_id: item.item_id.clone(),
                sku: item.sku.clone(),
                quantity: item.quantity,
                unit_price: item.unit_price.unwrap_or_default(),
            })
            .collect();
        let total_amount = items
            .iter()
            .map(|item| item.unit_price * f64::from(item.quantity))
            .sum();
        let order = Order {
            id: order_id,
            customer_id: body.customer_id.unwrap_or_default(),
            items,
            total_amount,
            created_at: Utc::now(),
            trace_id,
        };
        let expected_items = order.items.len();
        let timeout = Duration::from_millis(self.timeout_ms.load(Ordering::Acquire));
        Ok((
            context.with_timeout_limit(timeout),
            ProcessOrderSourceState {
                order,
                expected_items,
                results: Vec::new(),
                response_sent: false,
            },
        ))
    }

    async fn consume_message(
        &self,
        context: MessageContext,
        stream: StreamContext<Order, OrderState, String>,
        state: Arc<Mutex<ProcessOrderSourceState>>,
        _data: HandlerData,
        result_context: Arc<
            ResultContext<
                ProcessOrderSourceState,
                ProcessOrderRequest,
                ProcessOrderResponse,
                Order,
                OrderState,
                String,
            >,
        >,
    ) -> HandlerResult {
        let order_id = state.lock().await.order.id.clone();
        let done = Arc::clone(&result_context);
        result_context.set_result_callback(
            order_id,
            ResultCallback::new(
                move |_context,
                      _stream,
                      state: Arc<Mutex<ProcessOrderSourceState>>,
                      value: Payload<OrderState>,
                      data| {
                    let done = Arc::clone(&done);
                    Box::pin(async move {
                        let mut state = state.lock().await;
                        if state.response_sent {
                            return true;
                        }
                        if value.status != "TIMED_OUT" {
                            state.results.extend(value.confirmed_items.iter().cloned());
                            if state.results.len() < state.expected_items {
                                return false;
                            }
                        }
                        let status = if value.status == "TIMED_OUT" {
                            value.status.clone()
                        } else if state.results.iter().all(|item| item.reserved) {
                            "CONFIRMED".to_owned()
                        } else {
                            "PARTIALLY_CONFIRMED".to_owned()
                        };
                        let mut total_amount: f64 = state
                            .results
                            .iter()
                            .map(|item| item.unit_price * f64::from(item.requested_qty))
                            .sum();
                        if state.results.is_empty() {
                            total_amount = state.order.total_amount;
                        }
                        let response = ProcessOrderResponse {
                            order_id: Some(state.order.id.clone()),
                            status: Some(status),
                            total_amount: Some(total_amount),
                            processed_at: Some(Utc::now()),
                            confirmed_items: (!state.results.is_empty()).then(|| {
                                state
                                    .results
                                    .iter()
                                    .map(|item| ProcessOrderResponseItem {
                                        item_id: Some(item.item_id.clone()),
                                        sku: Some(item.sku.clone()),
                                        available_qty: Some(item.available_qty),
                                        reserved: Some(item.reserved),
                                        status: Some(item.status.clone()),
                                        error: (!item.error.is_empty()).then(|| item.error.clone()),
                                    })
                                    .collect()
                            }),
                        };
                        match serde_json::to_vec(&response) {
                            Ok(body) => {
                                data.set_status(StatusCode::OK);
                                data.set_header(
                                    CONTENT_TYPE,
                                    HeaderValue::from_static("application/json"),
                                );
                                data.set_response_body(body);
                                state.response_sent = true;
                                done.done();
                                true
                            }
                            Err(_) => false,
                        }
                    })
                },
            ),
        );
        let order = state.lock().await.order.clone();
        stream.collect(context, order).await;
        Ok(())
    }

    async fn get_message_id(
        &self,
        _context: &MessageContext,
        _stream: &StreamContext<Order, OrderState, String>,
        _state: Arc<Mutex<ProcessOrderSourceState>>,
        value: &OrderState,
    ) -> String {
        value.order_id.clone()
    }

    async fn end_request(
        &self,
        _context: MessageContext,
        _stream: StreamContext<Order, OrderState, String>,
        result: &HandlerResult,
        _state: Arc<Mutex<ProcessOrderSourceState>>,
        data: HandlerData,
    ) {
        if result.is_err() {
            data.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            data.set_header(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            data.set_response_body(br#"{"error":"internal server error"}"#.to_vec());
        }
    }
}

pub fn make_process_order_source(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &HttpEndpointConfig,
) -> RuntimeResult<ProcessOrderSource> {
    Ok(ProcessOrderSource {
        timeout_ms: Arc::new(AtomicU64::new(5_000)),
    })
}
