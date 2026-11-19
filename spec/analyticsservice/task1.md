# Task 1/26: `AnalyticsScheduleSource`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `schedule-source` |
| File | `analyticsservice/src/internal/functions/cron/analytics_schedule_source.rs` |
| Service | `Analytics Service` |


## Behaviour

Create an analytics job message identifying the local scheduled firing.





## Stream types
- Emits into graph: `AutomationJob` — `model_rust/src/types/automation_job.generated.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/cron/analytics_schedule_source.rs` and preserve its generated contract
- [ ] Inspect emitted message type `AutomationJob` in `model_rust/src/types/automation_job.generated.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task1.md — AnalyticsScheduleSource — Rust — done`