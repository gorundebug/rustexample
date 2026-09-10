# Task 12/22: `KeyOrdersForJoin`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `keyBy` |
| File | `analyticsservice/src/internal/functions/joinanalytics/key_orders_for_join.rs` |
| Service | `Analytics Service` |


## Behaviour

Key the order analytics event by correlation key.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Output: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Key: `AnalyticsKey` — `analyticsservice/src/internal/types/analytics_key.generated.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/joinanalytics/key_orders_for_join.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Inspect output type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task12.md — KeyOrdersForJoin — Rust — done`