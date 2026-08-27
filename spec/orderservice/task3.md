# Task 3/8: `ProcessOrderItemSink`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `grpc-sink` |
| File | `orderservice/src/internal/functions/endpoint/process_order_item_sink.rs` |
| Service | `Order Service` |


## Behaviour

Reserve inventory for one order item using its order ID, item ID, SKU, and quantity.
Return the available quantity, reservation outcome, and status. The caller combines this response with the original identity, requested quantity, and unit price.
If the inventory call fails, the caller returns a non-reserved PROCESSING_ERROR result with the failure message.




## External contract

| Field | Value |
|-------|-------|
| Format | `proto` |
| Source | `inventory_service_api/proto/inventoryserviceapi/processorderitem/processorderitem.proto` |
| Request | `ProcessOrderItemRequest` |
| Response | `ProcessOrderItemResponse` |


## Stream types
- Input: `OrderItem` — `model/src/types/order_item.rs`
- Output: `OrderItemResult` — `model/src/types/order_item_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `orderservice/src/internal/functions/endpoint/process_order_item_sink.rs` and preserve its generated contract
- [ ] Read `inventory_service_api/proto/inventoryserviceapi/processorderitem/processorderitem.proto`; change the source contract rather than generated bindings
- [ ] Inspect input type `OrderItem` in `model/src/types/order_item.rs`
- [ ] Inspect output type `OrderItemResult` in `model/src/types/order_item_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Verify the endpoint/result lifecycle, including completion and error paths
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] orderservice/task3.md — ProcessOrderItemSink — Rust — done`