//! Client network allocation DTOs.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AllocationAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub ip_alias: Option<String>,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub is_default: bool,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAllocationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
