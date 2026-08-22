# Task 6/8: `MapToOrderState`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `map` |
| File | `orderservice/src/internal/functions/map_to_order_state.rs` |
| Service | `Order Service` |


## Behaviour

Produce a TIMED_OUT order result that preserves the order ID and submitted total.
Do not add item results at this stage; results received before the timeout are included in the final response.





## Stream types
- Input: `Order` — `orderservice/src/internal/types/order.rs`
- Output: `OrderState` — `orderservice/src/internal/types/order_state.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `orderservice/src/internal/functions/map_to_order_state.rs` and preserve its generated contract
- [ ] Inspect input type `Order` in `orderservice/src/internal/types/order.rs`
- [ ] Inspect output type `OrderState` in `orderservice/src/internal/types/order_state.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] orderservice/task6.md — MapToOrderState — Rust — done`