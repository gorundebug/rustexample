# Task 1/2: `OrderProcessedEndpointSource`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `kafka-source` |
| File | `analyticsservice/src/internal/functions/endpoint/order_processed_endpoint_source.rs` |
| Service | `Analytics Service` |


## Behaviour

Exchange OrderProcessed events keyed by order ID.
Producers include the final status, processing time, total and confirmed item counts, and a failure reason for unsuccessful orders.
Consumers decode the event and mark its Kafka message processed only after the pipeline handles it successfully.





## Stream types
- Input: `OrderProcessed` — `model/src/types/order_processed.rs`
- Output: `OrderProcessed` — `model/src/types/order_processed.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `analyticsservice/src/internal/functions/endpoint/order_processed_endpoint_source.rs` and preserve its generated contract
- [ ] Inspect input type `OrderProcessed` in `model/src/types/order_processed.rs`
- [ ] Inspect output type `OrderProcessed` in `model/src/types/order_processed.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] analyticsservice/task1.md — OrderProcessedEndpointSource — Rust — done`