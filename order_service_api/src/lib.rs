#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    non_camel_case_types,
    unused_imports,
    unused_attributes
)]


pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.0.1";

#[path = "generated/header.generated.rs"]
pub(crate) mod header;
#[path = "generated/models.generated.rs"]
pub mod models;
#[path = "generated/types.generated.rs"]
pub mod types;

#[path = "generated/apis/mod.generated.rs"]
pub mod apis;

#[path = "generated/server.generated.rs"]
pub mod server;