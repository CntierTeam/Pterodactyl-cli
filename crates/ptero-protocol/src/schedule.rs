//! Client schedule / task DTOs.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScheduleAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub cron: Option<Value>,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub is_processing: bool,
    #[serde(default)]
    pub only_when_online: bool,
    #[serde(default)]
    pub last_run_at: Option<String>,
    #[serde(default)]
    pub next_run_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskAttributes {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub sequence_id: u64,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub payload: String,
    #[serde(default)]
    pub time_offset: u64,
    #[serde(default)]
    pub is_queued: bool,
    #[serde(default)]
    pub continue_on_failure: bool,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub minute: String,
    pub hour: String,
    pub day_of_month: String,
    pub month: String,
    pub day_of_week: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub only_when_online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub action: String,
    pub payload: String,
    pub time_offset: u64,
    #[serde(default)]
    pub continue_on_failure: bool,
}
