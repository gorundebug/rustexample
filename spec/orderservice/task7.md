# Task 7/8: `MapToOrderProcessed`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `map` |
| File | `orderservice/src/internal/functions/map_to_order_processed.rs` |
| Service | `Order Service` |


## Behaviour

Create an OrderProcessed event from the final order state.
Preserve the order ID, status, and processing time. Count all item results and reserved items; for unsuccessful orders use the final status as the failure reason.





## Stream types
- Input: `OrderState` — `orderservice/src/internal/types/order_state.rs`
- Output: `OrderProcessed` — `model/src/types/order_processed.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `orderservice/src/internal/functions/map_to_order_processed.rs` and preserve its generated contract
- [ ] Inspect input type `OrderState` in `orderservice/src/internal/types/order_state.rs`
- [ ] Inspect output type `OrderProcessed` in `model/src/types/order_processed.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] orderservice/task7.md — MapToOrderProcessed — Rust — done`