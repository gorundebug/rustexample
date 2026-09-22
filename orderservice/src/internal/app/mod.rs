mod service;
#[path = "imports.generated.rs"]
mod imports;
#[path = "makers.generated.rs"]
mod makers_generated;
#[path = "functions.generated.rs"]
mod functions_generated;
#[path = "streams.generated.rs"]
mod streams_generated;
#[path = "connectors.generated.rs"]
mod connectors_generated;
#[path = "clients.generated.rs"]
mod clients_generated;
#[path = "bindings.generated.rs"]
mod bindings_generated;
#[path = "endpoints.generated.rs"]
mod endpoints_generated;
#[path = "servers.generated.rs"]
mod servers_generated;
#[path = "substreams.generated.rs"]
mod substreams_generated;
#[path = "service.generated.rs"]
mod service_generated;

pub use service::Service;