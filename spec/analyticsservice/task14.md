# Task 14/17: `MultiJoinAnalyticsEvents`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `multiJoin` |
| File | `analyticsservice/src/internal/functions/multijoinanalytics/multi_join_analytics_events.rs` |
| Service | `Analytics Service` |


## Behaviour

Combine matching order, payment, and shipment analytics events.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Output: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`
- Key: `AnalyticsKey` — `analyticsservice/src/internal/types/analytics_key.generated.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/multijoinanalytics/multi_join_analytics_events.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Inspect output type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task14.md — MultiJoinAnalyticsEvents — Rust — done`