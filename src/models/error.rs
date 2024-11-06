use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ValidationError {
    EmptyValue,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyValue => write!(f, "値が空です"),
        }
    }
}

impl std::error::Error for ValidationError {}
