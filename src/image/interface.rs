use std::env;

use crate::models::{InputData, OutputData, OutputErrorData};

#[async_trait::async_trait]
pub trait GenerateImageStrategy: Send + Sync {
    async fn process(&self, input: &InputData) -> Result<OutputData, OutputErrorData>;

    fn get_env(&self, key: &str) -> Option<String> {
        match env::var(key) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}
