# Task 23/26: `BuildSubstreamAnalyticsResult`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `map` |
| File | `analyticsservice/src/internal/functions/substreamanalytics/build_substream_analytics_result.rs` |
| Service | `Analytics Service` |


## Behaviour

Transform one callable SubStream input into its analytics result.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Output: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/substreamanalytics/build_substream_analytics_result.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Inspect output type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task23.md — BuildSubstreamAnalyticsResult — Rust — done`