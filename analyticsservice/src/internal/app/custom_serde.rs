use std::any::TypeId;
use servicelib::runtime::{
    environment::{RuntimeEnvironment, RuntimeResult},
    serde::Serializer,
};

// User-owned override. Return None to use the generated or standard factory.
// For example, explicitly opt a type into JSON with:
// Serializer::new::<MyType>(Arc::new(JsonSerde::<MyType>::new())).
pub(super) fn get_custom_serde(
    _value_type: TypeId,
    _environment: &RuntimeEnvironment,
) -> RuntimeResult<Option<Serializer>> {
    Ok(None)
}