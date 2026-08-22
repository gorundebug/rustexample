# Implementation Rules

These rules apply to every `spec/*/task*.md`. The generated graph and transport
contracts are the source of truth; business implementations and their tests are
user-owned extension points.

## Project invariants

- Project root: `example/`
- Graph: `example/graph/example.generated.yaml`
- Never edit a file whose name contains `generated`; those files are replaced
  during project merge.
- Never change generated signatures, topology wiring, IDs, config keys, or
  transport contracts in order to make an implementation easier.
- Change `.proto`/OpenAPI source and regenerate; never patch generated bindings.
- Preserve the message/stream context received from the framework.
- Do not keep mutable per-request state in function objects: function instances
  are created once and may process requests concurrently.
- Finish one task at a time and immediately copy its completion line to
  `spec/progress.md`.

## Services

| Service | Language | Directory |
|---------|----------|-----------|
| `Analytics Service` | `Rust` | `analyticsservice/` |
| `Inventory Service` | `Rust` | `inventoryservice/` |
| `Order Service` | `Rust` | `orderservice/` |

## Rust rules

- Business functions are user-owned structs implementing the servicelib traits
  used by generated stream construction. Keep the trait methods and maker
  signatures unchanged.
- Propagate `MessageContext`, use the provided collectors and stream contexts,
  and await asynchronous operations. Do not block Tokio worker threads.
- Keep generated ownership boundaries: payload values and `Arc` state must not
  outlive the request or graph lifecycle without an explicit runtime contract.
- Use the generated workspace commands for build and test; the canonical test
  command is `cargo test --workspace --all-targets`.
- Put meaningful unit tests in `#[cfg(test)]` modules next to user-owned
  functions.
- Do not modify `service.generated.rs`, `config.generated.rs`, generated
  protobuf modules, or generated OpenAPI models.

## Endpoint and serialization rules

- External request/response types belong to protobuf/OpenAPI contracts.
- Internal stream types belong to the language backend's model package.
- Convert between external and internal types in endpoint handlers.
- Add serialization only where data crosses a process/storage boundary.
- For source endpoints, verify a real request and include the command in the
  task completion entry when the task asks for it.

## Priority of truth

1. Current task file.
2. Graph definition.
3. `.proto`/OpenAPI source contracts.
4. Generated type signatures.
5. servicelib runtime semantics for the selected language.
