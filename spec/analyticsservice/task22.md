# Task 22/22: `StandardAnalyticsSink`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `custom-sink` |
| File | `analyticsservice/src/internal/functions/endpoint/standard_analytics_sink.rs` |
| Service | `Analytics Service` |


## Behaviour

Validate and record analytics results routed to the standard Case branch.




## Stream types
- Input: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`
- Output: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/endpoint/standard_analytics_sink.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Inspect output type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task22.md — StandardAnalyticsSink — Rust — done`