# Inventory Service

Standalone generated Rust service.

```bash
make generate         # generate owned transport bindings
make build            # build this package
make test             # test this package
make lint             # rustfmt check and Clippy with warnings denied
make fmt              # format Rust sources
make clean            # remove Rust build artifacts
make docker-build     # build the autonomous runtime image from copied sources
make docker-up        # build and start only this service
make docker-up-dev    # start with this directory mounted read-only
make debug DEBUG_PORT=2345 # start gdbserver using this host port
make docker-down
make docker-down-dev
make docker-clean     # stop and remove standalone volumes
make help
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
The default application listeners are HTTP `9092`
and gRPC `9202`. The generated listener variables
also select the container side of each mapping; their `_HOST_HTTP_PORT` and
`_HOST_GRPC_PORT` counterparts change only host forwarding.