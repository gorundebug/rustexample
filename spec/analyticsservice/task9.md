# Task 9/22: `CompleteCycleAnalytics`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `filter` |
| File | `analyticsservice/src/internal/functions/cycleanalytics/complete_cycle_analytics.rs` |
| Service | `Analytics Service` |


## Behaviour

Keep the terminal analytics event once its cycle counter reaches three.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Output: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/cycleanalytics/complete_cycle_analytics.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Inspect output type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task9.md — CompleteCycleAnalytics — Rust — done`