// models/output.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputErrorData {
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Vec<u8>,
}

impl OutputData {
    pub fn new(result: Vec<u8>) -> Self {
        Self { result }
    }
}

impl OutputErrorData {
    pub fn new(error_message: String) -> Self {
        Self { error_message }
    }
}
