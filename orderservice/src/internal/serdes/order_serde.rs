// User-owned serializer. This file survives regeneration.
// Replace this alias with your Serde implementation, or explicitly select
// JsonSerde if JSON is the intended wire format for this type.
pub type OrderSerde = servicelib::runtime::serde::JsonSerde<crate::internal::types::Order>;