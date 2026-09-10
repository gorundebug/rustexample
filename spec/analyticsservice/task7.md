# Task 7/22: `CycleAnalyticsInputSource`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `custom-source` |
| File | `analyticsservice/src/internal/functions/endpoint/cycle_analytics_input_source.rs` |
| Service | `Analytics Service` |


## Behaviour

Produce one deterministic analytics event that exercises the finite feedback cycle.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/endpoint/cycle_analytics_input_source.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task7.md — CycleAnalyticsInputSource — Rust — done`