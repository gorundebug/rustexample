# Task 4/8: `MapOrderItemResultToOrderState`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `map` |
| File | `orderservice/src/internal/functions/map_order_item_result_to_order_state.rs` |
| Service | `Order Service` |


## Behaviour

Produce an order result containing one inventory result and preserving its order ID.
Mark it CONFIRMED when the item was reserved; otherwise mark it PARTIALLY_CONFIRMED.
Record the time when this result is produced.





## Stream types
- Input: `OrderItemResult` — `model/src/types/order_item_result.rs`
- Output: `OrderState` — `orderservice/src/internal/types/order_state.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `orderservice/src/internal/functions/map_order_item_result_to_order_state.rs` and preserve its generated contract
- [ ] Inspect input type `OrderItemResult` in `model/src/types/order_item_result.rs`
- [ ] Inspect output type `OrderState` in `orderservice/src/internal/types/order_state.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] orderservice/task4.md — MapOrderItemResultToOrderState — Rust — done`