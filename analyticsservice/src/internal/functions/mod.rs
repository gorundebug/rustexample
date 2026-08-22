// User-owned function modules are added here by the merge workflow.
mod count_order_processed;
mod order_processed_endpoint;

pub use count_order_processed::{CountOrderProcessed, make_count_order_processed};
pub use order_processed_endpoint::{OrderProcessedEndpoint, make_order_processed_endpoint};
