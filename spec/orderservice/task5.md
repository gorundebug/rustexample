# Task 5/8: `SoftDeadline`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `delay` |
| File | `orderservice/src/internal/functions/soft_deadline.rs` |
| Service | `Order Service` |


## Behaviour

Trigger the timeout branch shortly before the request deadline, leaving the configured duration to assemble a response.
When no request deadline exists, use the configured duration itself. Never wait past an existing deadline.





## Stream types
- Input: `Order` — `orderservice/src/internal/types/order.rs`
- Output: `Order` — `orderservice/src/internal/types/order.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `orderservice/src/internal/functions/soft_deadline.rs` and preserve its generated contract
- [ ] Inspect input type `Order` in `orderservice/src/internal/types/order.rs`
- [ ] Inspect output type `Order` in `orderservice/src/internal/types/order.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] orderservice/task5.md — SoftDeadline — Rust — done`