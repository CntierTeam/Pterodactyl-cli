use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type PteroResult<T> = Result<T, PteroError>;

/// Panel JSON:API-style error body: `{ "errors": [ { code, status, detail } ] }`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiError {
    #[serde(default)]
    pub errors: Vec<ApiErrorItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiErrorItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl ApiError {
    pub fn primary_message(&self) -> String {
        self.errors
            .first()
            .map(|e| {
                if !e.detail.is_empty() {
                    e.detail.clone()
                } else if !e.code.is_empty() {
                    e.code.clone()
                } else {
                    "unknown API error".into()
                }
            })
            .unwrap_or_else(|| "empty errors array".into())
    }

    pub fn http_status_hint(&self) -> i32 {
        self.errors
            .first()
            .and_then(|e| e.status.parse::<i32>().ok())
            .unwrap_or(0)
    }
}

#[derive(Debug, Error)]
pub enum PteroError {
    #[error("API error {status}: {message}")]
    Api { status: i32, message: String },

    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("not configured: {0}")]
    Config(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("serialization error: {0}")]
    Serde(String),

    #[error("wings / websocket error: {0}")]
    Wings(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("{0}")]
    Message(String),
}

impl PteroError {
    pub fn from_api_body(http_status: i32, body: &str) -> Self {
        if let Ok(err) = serde_json::from_str::<ApiError>(body) {
            if !err.errors.is_empty() {
                let hint = err.http_status_hint();
                let status = if hint != 0 { hint } else { http_status };
                return PteroError::Api {
                    status,
                    message: err.primary_message(),
                };
            }
        }
        PteroError::Api {
            status: http_status,
            message: if body.is_empty() {
                "empty error body".into()
            } else {
                body.to_string()
            },
        }
    }
}

impl From<serde_json::Error> for PteroError {
    fn from(e: serde_json::Error) -> Self {
        PteroError::Serde(e.to_string())
    }
}

impl From<std::io::Error> for PteroError {
    fn from(e: std::io::Error) -> Self {
        PteroError::Io(e.to_string())
    }
}

/// Serializable error for JSON CLI output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBody {
    pub status: i32,
    pub message: String,
}
