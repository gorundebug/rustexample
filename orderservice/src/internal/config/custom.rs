use serde::{Deserialize, Serialize};

/// User-owned service configuration. This file is preserved by generated merges.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CustomConfig {}

impl CustomConfig {
    pub fn apply_environment(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
