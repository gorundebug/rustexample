# Task 25/26: `InvokeAnalyticsSubstream`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `map` |
| File | `analyticsservice/src/internal/functions/substreamanalytics/invoke_analytics_substream.rs` |
| Service | `Analytics Service` |


## Behaviour

Invoke the service-local analytics SubStream and emit its returned result.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Output: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/substreamanalytics/invoke_analytics_substream.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Inspect output type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task25.md — InvokeAnalyticsSubstream — Rust — done`