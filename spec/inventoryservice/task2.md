# Task 2/2: `GetInventoryItemData`

> Rules: [`spec/rules.md`](../rules.md)

| Field | Value |
|-------|-------|
| Language | `Rust` |
| Kind | `process` |
| File | `inventoryservice/src/internal/functions/get_inventory_item_data.rs` |
| Service | `Inventory Service` |


## Behaviour

Reserve the requested quantity without allowing concurrent orders to overdraw stock.
On success, return CONFIRMED with the requested quantity available. Otherwise return OUT_OF_STOCK with the current available quantity.
Preserve the order and item identity, requested quantity, and unit price.
The example starts with SKU-001: 100, SKU-002: 50, and SKU-003: 25.





## Stream types
- Input: `OrderItem` — `model/src/types/order_item.rs`
- Output: `OrderItemResult` — `model/src/types/order_item_result.rs`

## Checklist

- [ ] Read [`spec/rules.md`](../rules.md), especially the `Rust` section
- [ ] Open `inventoryservice/src/internal/functions/get_inventory_item_data.rs` and preserve its generated contract
- [ ] Inspect input type `OrderItem` in `model/src/types/order_item.rs`
- [ ] Inspect output type `OrderItemResult` in `model/src/types/order_item_result.rs`
- [ ] Implement the Rust function without changing its generated trait contract
- [ ] Preserve `MessageContext` and await collector, sender and result operations
- [ ] Add meaningful `#[cfg(test)]` coverage in the user-owned function module
- [ ] Run `cargo test --workspace --all-targets`
- [ ] Re-read this checklist
- [ ] Append to `spec/progress.md`: `- [x] inventoryservice/task2.md — GetInventoryItemData — Rust — done`