# Task 2/8: `ProcessOrderItems`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `flatMap` |
| File | `orderservice/src/internal/functions/order/process_order_items.rs` |
| Service | `Order Service` |


## Behaviour

Emit every order item independently for inventory processing.
Preserve each item's data and assign the parent order ID.





## Stream types
- Input: `Order` — `orderservice/src/internal/types/order.rs`
- Output: `OrderItem` — `model/src/types/order_item.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `orderservice/src/internal/functions/order/process_order_items.rs` and preserve its generated contract
- [ ] Inspect input type `Order` in `orderservice/src/internal/types/order.rs`
- [ ] Inspect output type `OrderItem` in `model/src/types/order_item.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] orderservice/task2.md — ProcessOrderItems — Rust — done`