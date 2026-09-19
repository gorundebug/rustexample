# Task 26/26: `SubstreamAnalyticsResultSink`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `custom-sink` |
| File | `analyticsservice/src/internal/functions/endpoint/substream_analytics_result_sink.rs` |
| Service | `Analytics Service` |


## Behaviour

Validate and record the result returned by the service-local SubStream example.




## Stream types
- Input: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`
- Output: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/endpoint/substream_analytics_result_sink.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Inspect output type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task26.md — SubstreamAnalyticsResultSink — Rust — done`