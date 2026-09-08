//! Client backup DTOs.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupAttributes {
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub is_successful: bool,
    #[serde(default)]
    pub is_locked: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub ignored_files: Vec<String>,
    #[serde(default)]
    pub checksum: Option<String>,
    #[serde(default)]
    pub bytes: u64,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBackupRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestoreBackupRequest {
    #[serde(default)]
    pub truncate: bool,
}
