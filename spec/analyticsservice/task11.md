# Task 11/22: `CycleAnalyticsResultSink`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `custom-sink` |
| File | `analyticsservice/src/internal/functions/endpoint/cycle_analytics_result_sink.rs` |
| Service | `Analytics Service` |


## Behaviour

Validate the terminal event emitted after three passes through the feedback cycle.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Output: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/endpoint/cycle_analytics_result_sink.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Inspect output type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task11.md — CycleAnalyticsResultSink — Rust — done`