use serde::{Deserialize, Serialize};

/// Input for the canonical analytics joins. Fields: Key AnalyticsKey, Value int, Kind string.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AnalyticsEvent {
    pub key: String,
    pub value: i32,
    pub kind: String,
}
