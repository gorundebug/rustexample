# Task 1/8: `ProcessOrder`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `http-source` |
| File | `orderservice/src/internal/functions/process_order.rs` |
| Service | `Order Service` |


## Behaviour

Accept orders with at least one item and positive quantities; reject malformed or invalid requests as client errors.
Reuse X-Request-ID when supplied, otherwise generate an order ID. Preserve customer, item, price, and X-Trace data, and apply the configured timeout of five seconds by default.
Return one response per order. When all items finish, use CONFIRMED only if every item was reserved; otherwise use PARTIALLY_CONFIRMED. If the deadline wins, return TIMED_OUT with the item results received so far.
Calculate the total from processed item prices, falling back to the submitted total when no item result arrived, and include individual item failures in the response.




## External contract

| Field | Value |
|-------|-------|
| Format | `openapi` |
| Source | `order_service_api/openapi/orderserviceapi/processorder/processorder.yaml` |
| Request | `ProcessOrderRequest` |
| Response | `ProcessOrderResponse` |


## Stream types
- Input: `Order` — `orderservice/src/internal/types/order.rs`
- Output: `OrderState` — `orderservice/src/internal/types/order_state.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `orderservice/src/internal/functions/process_order.rs` and preserve its generated contract
- [ ] Read `order_service_api/openapi/orderserviceapi/processorder/processorder.yaml`; change the source contract rather than generated bindings
- [ ] Inspect input type `Order` in `orderservice/src/internal/types/order.rs`
- [ ] Inspect output type `OrderState` in `orderservice/src/internal/types/order_state.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Verify the endpoint/result lifecycle, including completion and error paths
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] orderservice/task1.md — ProcessOrder — Rust — done`