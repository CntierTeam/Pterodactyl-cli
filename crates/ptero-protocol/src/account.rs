//! Client account / API key / SSH key / activity DTOs.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub admin: bool,
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
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeyAttributes {
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub allowed_ips: Vec<String>,
    #[serde(default)]
    pub last_used_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    /// Present only on create responses.
    #[serde(default)]
    pub secret_token: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SshKeyAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub public_key: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityAttributes {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub batch: Option<String>,
    #[serde(default)]
    pub event: String,
    #[serde(default)]
    pub is_api: bool,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub properties: Option<Value>,
    #[serde(default)]
    pub has_additional_metadata: Option<bool>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TwoFactorData {
    #[serde(default)]
    pub data_url_image: Option<String>,
    #[serde(default)]
    pub image_url_data: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmailRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnableTwoFactorRequest {
    pub code: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableTwoFactorRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSshKeyRequest {
    pub name: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSshKeyRequest {
    pub fingerprint: String,
}
