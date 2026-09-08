//! Shared protocol types for Pterodactyl Panel HTTP and Wings WebSocket.
//!
//! This crate is the **only** data contract between `ptero-panel`, `ptero-wings`,
//! `ptero-core`, `ptero-cli`, and `ptero-tui`.

pub mod account;
pub mod application;
pub mod backup;
pub mod error;
pub mod files;
pub mod network;
pub mod remote;
pub mod schedule;
pub mod server;
pub mod ws;

pub use error::{ApiError, ApiErrorItem, ErrorBody, PteroError, PteroResult};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Fractal single-resource envelope: `{ object, attributes, relationships? }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalItem<T> {
    pub object: String,
    pub attributes: T,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Value>,
}

/// Fractal list envelope: `{ object, data, meta? }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalList<T> {
    pub object: String,
    pub data: Vec<FractalItem<T>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// Optional Fractal meta block.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Meta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
    /// Extra meta keys (startup command, docker images, secret token, …).
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// Fractal pagination under `meta.pagination`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pagination {
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub per_page: u64,
    #[serde(default)]
    pub current_page: u64,
    #[serde(default)]
    pub total_pages: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<PaginationLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// Websocket credentials from Client websocket endpoint.
///
/// Panel returns `{ "data": { "token", "socket" } }` (not Fractal attributes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketCredentials {
    pub token: String,
    pub socket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketResponse {
    pub data: WebsocketCredentials,
}

impl WebsocketResponse {
    pub fn into_credentials(self) -> WebsocketCredentials {
        self.data
    }
}
