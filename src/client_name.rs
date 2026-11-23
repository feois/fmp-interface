
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ClientNameError {
    #[error("Clashed name")]
    Clashed,
    #[error("Empty name")]
    Empty,
    #[error("Invalid name")]
    Invalid,
}

pub fn validate(name: &str) -> Option<ClientNameError> {
    if name.is_empty() { Some(ClientNameError::Empty) }
    else if !name.chars().all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | ' ' | '_' | '-' | '0'..='9')) { Some(ClientNameError::Invalid) }
    else { None }
}
