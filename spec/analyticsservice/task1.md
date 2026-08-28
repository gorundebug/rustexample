# Task 1/3: `AnalyticsScheduleSource`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `schedule-source` |
| File | `analyticsservice/src/internal/functions/cron/analytics_schedule_source.rs` |
| Service | `Analytics Service` |


## Behaviour

Create an analytics job message identifying the local scheduled firing.





## Stream types
- Input: `AutomationJob` — `model_rust/src/types/automation_job.generated.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/cron/analytics_schedule_source.rs` and preserve its generated contract
- [ ] Inspect input type `AutomationJob` in `model_rust/src/types/automation_job.generated.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task1.md — AnalyticsScheduleSource — Rust — done`