# Task 6/26: `AnalyticsShipmentsSource`

> Rules: [`spec/rules.md`](../rules.md)

Paths and commands below are relative to the generated project root.

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `custom-source` |
| File | `analyticsservice/src/internal/functions/endpoint/analytics_shipments_source.rs` |
| Service | `Analytics Service` |


## Behaviour

Produce a deterministic shipment analytics event for the canonical multi-way join example.




## Stream types
- Emits into graph: `AnalyticsEvent` — `analyticsservice/src/internal/types/analytics_event.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/endpoint/analytics_shipments_source.rs` and preserve its generated contract
- [ ] Inspect emitted message type `AnalyticsEvent` in `analyticsservice/src/internal/types/analytics_event.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`

- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task6.md — AnalyticsShipmentsSource — Rust — done`