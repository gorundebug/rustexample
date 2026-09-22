// Domain model types are separate from OpenAPI's transport helper types.
#[path = "types/mod.rs"]
pub mod model_types;
pub use model_types as types;