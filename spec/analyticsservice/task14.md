# Task 14/22: `JoinOrderPaymentAnalytics`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `join` |
| File | `analyticsservice/src/internal/functions/joinanalytics/join_order_payment_analytics.rs` |
| Service | `Analytics Service` |


## Behaviour

Join matching order and payment analytics events and emit their combined total.




## Stream types
- Input: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`
- Output: `AnalyticsResult` — `analyticsservice/src/internal/types/analytics_result.rs`
- Key: `AnalyticsKey` — `analyticsservice/src/internal/types/analytics_key.generated.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/joinanalytics/join_order_payment_analytics.rs` and preserve its generated contract
- [ ] Inspect input type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Inspect output type `AnalyticsResult` in `analyticsservice/src/internal/types/analytics_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task14.md — JoinOrderPaymentAnalytics — Rust — done`