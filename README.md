# Rust example

The Rust implementation of the same topology and business behavior as
[`goexample`](../goexample). Each service and contract module is an independent
Cargo package and may be published from its own repository.

The Go implementation is the semantic reference.

## Run

```sh
make docker-up
```

For the clean production-style layout use `make docker-up RUNTIME_IMAGE=1`.
It selects the final multi-stage runtime target without sources or build tools;
benchmark and profiling tools select this mode automatically. Rust services
already use the final runtime stage by default, so the flag is intentionally
compatible without changing their runtime behavior.

## Optional order analytics through Kafka

The shared `orderProcessed` Kafka endpoint is disabled in Order Service by
default and creates no producer while disabled. Enable it in
`orderservice/config/overrides.yaml`:

```yaml
endpoints:
  orderProcessed:
    enabled: true
```

The Analytics Service consumes `order-processed`, counts successful and
unsuccessful orders, and uses the included Redpanda broker.

Submit an order with at least one in-stock SKU:

```sh
curl --fail-with-body \
  -H 'Content-Type: application/json' \
  -d '{"customer_id":"customer-1","items":[{"item_id":"item-1","sku":"SKU-001","quantity":2,"unit_price":10}]}' \
  http://localhost:9091/v1/processorder
```

Status and metrics:

```sh
curl http://localhost:9091/status
curl http://localhost:9091/metrics
curl http://localhost:9092/status
curl http://localhost:9092/metrics
```

Stop the workspace:

```sh
make docker-down
```
