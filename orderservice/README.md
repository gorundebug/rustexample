# Order Service

Standalone generated Rust service.

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
make docker-build     # [Docker] build the autonomous runtime image from copied sources
make docker-up        # [Docker] build and start only this service
make docker-up-dev    # [Docker] start with this directory mounted read-only
make debug DEBUG_PORT=2345 # [Docker] start gdbserver using this host port
make docker-down      # [Docker] stop the standalone runtime stack
make docker-down-dev  # [Docker] stop the standalone development stack
make docker-clean     # [Docker] stop and remove standalone volumes
make help             # [host] list generated targets
```
The service defaults to pinned repository crates (`USE_LOCAL_MODULES=0`). A
project workspace passes `USE_LOCAL_MODULES=1`. The same explicit local mode is
supported when the service and its unpublished modules were obtained separately
and placed next to one another using their generated directory names:

```bash
make build USE_LOCAL_MODULES=1
make docker-build USE_LOCAL_MODULES=1
```

After publishing and pinning the crates, omit the flag or pass
`USE_LOCAL_MODULES=0`. Make does not infer the mode from the filesystem.
Dependency proxy selection is an independent caller environment concern via
`DEPENDENCY_PROXY_DIR`.
`gdbserver` always listens on `2345` inside the container; `DEBUG_PORT` selects
the forwarded host port.
The default application listeners are HTTP `9091`
and gRPC `9201`. The generated listener variables
also select the container side of each mapping; their `_HOST_HTTP_PORT` and
`_HOST_GRPC_PORT` counterparts change only host forwarding.