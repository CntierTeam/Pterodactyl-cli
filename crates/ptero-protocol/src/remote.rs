//! Remote (`/api/remote`) DTOs — Wings↔Panel internal API.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteResource {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SftpAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SftpAuthResponse {
    #[serde(default)]
    pub server: Option<String>,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteInstallDetails {
    #[serde(default)]
    pub container_image: Option<String>,
    #[serde(default)]
    pub entrypoint: Option<String>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteInstallCompleteRequest {
    #[serde(default)]
    pub successful: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteBackupStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteBackupRestoreRequest {
    #[serde(default)]
    pub successful: bool,
}
