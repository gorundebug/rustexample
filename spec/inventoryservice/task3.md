# Task 3/3: `GetInventoryItemError`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `map` |
| File | `inventoryservice/src/internal/functions/inventory_item/get_inventory_item_error.rs` |
| Service | `Inventory Service` |


## Behaviour

When inventory processing fails, return an OUT_OF_STOCK result with no available quantity.
Preserve the order and item identity and requested quantity, and record the failure.





## Stream types
- Input: `InventoryFailure` — `inventoryservice/src/internal/types/inventory_failure.rs`
- Output: `OrderItemResult` — `model_rust/src/types/order_item_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `inventoryservice/src/internal/functions/inventory_item/get_inventory_item_error.rs` and preserve its generated contract
- [ ] Inspect input type `InventoryFailure` in `inventoryservice/src/internal/types/inventory_failure.rs`
- [ ] Inspect output type `OrderItemResult` in `model_rust/src/types/order_item_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] inventoryservice/task3.md — GetInventoryItemError — Rust — done`