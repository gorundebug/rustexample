use std::{
    collections::HashMap,
    sync::atomic::{AtomicI32, Ordering},
};

use async_trait::async_trait;
use example_model::types::{OrderItem, OrderItemResult};
use servicelib::{
    Collector, MessageContext,
    operators::process::ProcessFunction,
    runtime::{
        common::RuntimeStream,
        config::ProcessStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

pub struct GetInventoryItemData {
    stock: HashMap<String, AtomicI32>,
}

impl Default for GetInventoryItemData {
    fn default() -> Self {
        Self {
            stock: HashMap::from([
                ("SKU-001".to_owned(), AtomicI32::new(100)),
                ("SKU-002".to_owned(), AtomicI32::new(50)),
                ("SKU-003".to_owned(), AtomicI32::new(25)),
            ]),
        }
    }
}

#[async_trait]
impl ProcessFunction<OrderItem, OrderItemResult, OrderItemResult> for GetInventoryItemData {
    async fn process(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &OrderItem,
        out: &Collector<OrderItemResult>,
        error: &Collector<OrderItemResult>,
    ) {
        let (available, reserved) = reserve(self.stock.get(&value.sku), value.quantity);
        let result = OrderItemResult {
            order_id: value.order_id.clone(),
            item_id: value.item_id.clone(),
            sku: value.sku.clone(),
            requested_qty: value.quantity,
            available_qty: available,
            reserved,
            status: if reserved {
                "CONFIRMED".to_owned()
            } else {
                "OUT_OF_STOCK".to_owned()
            },
            unit_price: value.unit_price,
            error: String::new(),
        };
        if reserved {
            out.collect(context, result).await;
        } else {
            error.collect(context, result).await;
        }
    }
}

fn reserve(stock: Option<&AtomicI32>, quantity: i32) -> (i32, bool) {
    let Some(stock) = stock else {
        return (0, false);
    };
    let mut available = stock.load(Ordering::Relaxed);
    while available >= quantity {
        match stock.compare_exchange_weak(
            available,
            available - quantity,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return (quantity, true),
            Err(actual) => available = actual,
        }
    }
    (available, false)
}

pub async fn make_get_inventory_item_data(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &ProcessStreamConfig,
) -> RuntimeResult<GetInventoryItemData> {
    Ok(GetInventoryItemData::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reservation_is_atomic_and_never_overdraws() {
        let stock = AtomicI32::new(100);
        std::thread::scope(|scope| {
            for _ in 0..200 {
                scope.spawn(|| {
                    reserve(Some(&stock), 1);
                });
            }
        });
        assert_eq!(stock.load(Ordering::Relaxed), 0);
    }
}
