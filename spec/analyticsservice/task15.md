# Task 15/17: `RouteAnalyticsResult`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `case` |
| File | `analyticsservice/src/internal/functions/multijoinanalytics/route_analytics_result.rs` |
| Service | `Analytics Service` |


## Behaviour

Route high-value analytics results to the first branch and all others to the second branch.




## Stream types
- Input: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`
- Output: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`
- Key: `AnalyticsKey` — `analyticsservice/src/internal/types/analytics_key.generated.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/multijoinanalytics/route_analytics_result.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Inspect output type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task15.md — RouteAnalyticsResult — Rust — done`