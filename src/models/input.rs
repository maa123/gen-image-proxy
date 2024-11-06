use serde::{Deserialize, Serialize};

use crate::models::error::ValidationError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputData {
    pub prompt: String,
    // デフォルト値: 6
    #[serde(default = "default_steps")]
    pub steps: u32,
}

fn default_steps() -> u32 {
    6
}

impl InputData {
    pub fn new(prompt: String, steps: Option<u32>) -> Self {
        Self {
            prompt,
            steps: steps.unwrap_or(default_steps()),
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.prompt.is_empty() {
            return Err(ValidationError::EmptyValue);
        }
        Ok(())
    }
}
