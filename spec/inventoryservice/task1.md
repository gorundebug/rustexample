# Task 1/2: `ProcessOrderItem`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `grpc-source` |
| File | `inventoryservice/src/internal/functions/process_order_item.rs` |
| Service | `Inventory Service` |


## Behaviour

Reserve inventory for one order item using its order ID, item ID, SKU, and quantity.
Return the available quantity, reservation outcome, and status. The caller combines this response with the original identity, requested quantity, and unit price.
If the inventory call fails, the caller returns a non-reserved PROCESSING_ERROR result with the failure message.





## Stream types
- Input: `OrderItem` — `model/src/types/order_item.rs`
- Output: `OrderItemResult` — `model/src/types/order_item_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `inventoryservice/src/internal/functions/process_order_item.rs` and preserve its generated contract
- [ ] Inspect input type `OrderItem` in `model/src/types/order_item.rs`
- [ ] Inspect output type `OrderItemResult` in `model/src/types/order_item_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] inventoryservice/task1.md — ProcessOrderItem — Rust — done`