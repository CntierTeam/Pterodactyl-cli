//! Application API DTOs.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppUserAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub root_admin: bool,
    #[serde(rename = "2fa", default)]
    pub two_factor: bool,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAppUserRequest {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppNodeAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub public: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub location_id: u64,
    #[serde(default)]
    pub fqdn: String,
    #[serde(default)]
    pub scheme: String,
    #[serde(default)]
    pub behind_proxy: bool,
    #[serde(default)]
    pub maintenance_mode: bool,
    #[serde(default)]
    pub memory: u64,
    #[serde(default)]
    pub memory_overallocate: i64,
    #[serde(default)]
    pub disk: u64,
    #[serde(default)]
    pub disk_overallocate: i64,
    #[serde(default)]
    pub upload_size: u64,
    #[serde(default)]
    pub daemon_listen: u16,
    #[serde(default)]
    pub daemon_sftp: u16,
    #[serde(default)]
    pub daemon_base: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppLocationAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub short: String,
    #[serde(default)]
    pub long: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub short: String,
    pub long: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppServerAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub suspended: bool,
    #[serde(default)]
    pub limits: Option<Value>,
    #[serde(default)]
    pub feature_limits: Option<Value>,
    #[serde(default)]
    pub user: u64,
    #[serde(default)]
    pub node: u64,
    #[serde(default)]
    pub allocation: u64,
    #[serde(default)]
    pub nest: u64,
    #[serde(default)]
    pub egg: u64,
    #[serde(default)]
    pub container: Option<Value>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppNestAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppEggAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nest: u64,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub docker_image: Option<String>,
    #[serde(default)]
    pub docker_images: Option<Value>,
    #[serde(default)]
    pub config: Option<Value>,
    #[serde(default)]
    pub startup: Option<String>,
    #[serde(default)]
    pub script: Option<Value>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppAllocationAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub assigned: bool,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppDatabaseAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub server: u64,
    #[serde(default)]
    pub host: u64,
    #[serde(default)]
    pub database: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub remote: String,
    #[serde(default)]
    pub max_connections: Option<u64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Generic Application resource when a dedicated type is unnecessary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationResource {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
