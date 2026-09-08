//! Client server DTOs.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerAttributes {
    #[serde(default)]
    pub server_owner: bool,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub internal_id: Option<u64>,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub node: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub is_suspended: bool,
    #[serde(default)]
    pub is_installing: bool,
    #[serde(default)]
    pub is_transferring: bool,
    #[serde(default)]
    pub limits: Option<Value>,
    #[serde(default)]
    pub feature_limits: Option<Value>,
    #[serde(default)]
    pub invocation: Option<String>,
    #[serde(default)]
    pub docker_image: Option<String>,
    #[serde(default)]
    pub sftp_details: Option<Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceStats {
    #[serde(default)]
    pub current_state: String,
    #[serde(default)]
    pub is_suspended: bool,
    #[serde(default)]
    pub resources: ResourceUtilization,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUtilization {
    #[serde(default)]
    pub memory_bytes: u64,
    #[serde(default)]
    pub cpu_absolute: f64,
    #[serde(default)]
    pub disk_bytes: u64,
    #[serde(default)]
    pub network_rx_bytes: u64,
    #[serde(default)]
    pub network_tx_bytes: u64,
    #[serde(default)]
    pub uptime: u64,
}

/// Power signal values accepted by `POST .../power`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PowerSignal {
    Start,
    Stop,
    Restart,
    Kill,
}

impl PowerSignal {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Restart => "restart",
            Self::Kill => "kill",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "start" | "o" => Some(Self::Start),
            "stop" | "s" => Some(Self::Stop),
            "restart" | "r" => Some(Self::Restart),
            "kill" | "k" => Some(Self::Kill),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerRequest {
    pub signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseAttributes {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub host: Option<Value>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub connections_from: Option<String>,
    #[serde(default)]
    pub max_connections: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDatabaseRequest {
    pub database: String,
    pub remote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubuserAttributes {
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub two_factor_enabled: bool,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubuserRequest {
    pub email: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartupMeta {
    #[serde(default)]
    pub startup_command: Option<String>,
    #[serde(default)]
    pub docker_images: Option<Value>,
    #[serde(default)]
    pub raw_startup_command: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EggVariableAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub env_variable: String,
    #[serde(default)]
    pub default_value: String,
    #[serde(default)]
    pub server_value: Option<String>,
    #[serde(default)]
    pub is_editable: bool,
    #[serde(default)]
    pub rules: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStartupVariableRequest {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameServerRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerImageRequest {
    pub docker_image: String,
}

/// Client permissions blob from `GET /api/client/permissions`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientPermissions {
    #[serde(default)]
    pub permissions: HashMap<String, Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
