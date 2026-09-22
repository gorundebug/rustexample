# model

Standalone generated Rust contract/model package.

## Message serializers

Serializers are selected by the service, not inferred from `serde::Serialize`.
The runtime caches them by Rust type within a service. The generated
`app/serde.generated.rs` resolver first calls the user-owned
`app/custom_serde.rs` hook, then the model's serializer factories. Returning
`None` allows standard primitive serializers and, finally, a stub fallback.
Types without a serializer can still flow through in-memory operators;
attempting to serialize them reports a serde error. JSON is opt-in.

Local implementations live in `src/internal/serdes`; shared model crates
export their implementations from `src/serdes`. Individual serializer files
survive regeneration. Their initial `StubSerde<T>` alias can be replaced with
an implementation of `servicelib::runtime::serde::Serde<T>` exposing `new()`.
To explicitly use JSON, select `JsonSerde<T>` instead. KeyBy resolves key and
value serializers through the same service mechanism. This is separate from
YAML configuration loading and HTTP/gRPC transport codecs.

```bash
make generate         # [host] generate owned transport bindings
make build            # [host] build this package
make test             # [host] test this package
make lint             # [host] rustfmt check and Clippy with warnings denied
make fmt              # [host] format Rust sources
make clean            # [host] remove Rust build artifacts
make help             # [host] list generated targets
```