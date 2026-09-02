# Order Service

Standalone generated Rust service.

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