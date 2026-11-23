
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::client_name::ClientNameError;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum IdentificationError {
    #[error("Clashed name")]
    Clashed,
    #[error("Empty name")]
    Empty,
    #[error("Invalid name")]
    Invalid,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Identified,
    IdentificationError(ClientNameError),
    Terminated,
}

impl Message {
    #[inline(always)]
    pub fn to_line(&self) -> String {
        let mut s = self.to_string();
        s.push('\n');
        s
    }
}

impl ToString for Message {
    #[inline(always)]
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for Message {
    type Err = serde_json::Error;
    
    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
