# Task 2/2: `CountOrderProcessed`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `process` |
| File | `analyticsservice/src/internal/functions/analytics/count_order_processed.rs` |
| Service | `Analytics Service` |


## Behaviour

Count successful and unsuccessful orders independently, then return the event unchanged.





## Stream types
- Input: `OrderProcessed` — `model/src/types/order_processed.rs`
- Output: `OrderProcessed` — `model/src/types/order_processed.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/analytics/count_order_processed.rs` and preserve its generated contract
- [ ] Inspect input type `OrderProcessed` in `model/src/types/order_processed.rs`
- [ ] Inspect output type `OrderProcessed` in `model/src/types/order_processed.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task2.md — CountOrderProcessed — Rust — done`