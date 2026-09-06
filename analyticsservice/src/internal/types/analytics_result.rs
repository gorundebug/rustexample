use serde::{Deserialize, Serialize};

/// Output of the canonical analytics joins. Fields: Key AnalyticsKey, Total int, Kind string.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AnalyticsResult {
    pub key: String,
    pub total: i32,
    pub kind: String,
}
