//! Wings console WebSocket event and request constants.
//!
//! Source: panel frontend `resources/scripts/components/server/events.ts`
//! and `resources/scripts/plugins/Websocket.ts`.

/// Client → Wings request event names.
pub mod request {
    pub const AUTH: &str = "auth";
    pub const SEND_COMMAND: &str = "send command";
    pub const SET_STATE: &str = "set state";
    pub const SEND_LOGS: &str = "send logs";
    pub const SEND_STATS: &str = "send stats";
}

/// Wings → Client event names.
pub mod event {
    pub const AUTH_SUCCESS: &str = "auth success";
    pub const TOKEN_EXPIRING: &str = "token expiring";
    pub const TOKEN_EXPIRED: &str = "token expired";
    pub const JWT_ERROR: &str = "jwt error";
    pub const DAEMON_MESSAGE: &str = "daemon message";
    pub const DAEMON_ERROR: &str = "daemon error";
    pub const INSTALL_OUTPUT: &str = "install output";
    pub const INSTALL_STARTED: &str = "install started";
    pub const INSTALL_COMPLETED: &str = "install completed";
    pub const CONSOLE_OUTPUT: &str = "console output";
    pub const STATUS: &str = "status";
    pub const STATS: &str = "stats";
    pub const TRANSFER_LOGS: &str = "transfer logs";
    pub const TRANSFER_STATUS: &str = "transfer status";
    pub const BACKUP_COMPLETED: &str = "backup completed";
    pub const BACKUP_RESTORE_COMPLETED: &str = "backup restore completed";
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Wire message: `{ "event": "...", "args": [...] }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub event: String,
    #[serde(default)]
    pub args: Vec<Value>,
}

impl WsMessage {
    pub fn new(event: impl Into<String>, args: Vec<Value>) -> Self {
        Self {
            event: event.into(),
            args,
        }
    }

    pub fn auth(token: &str) -> Self {
        Self::new(request::AUTH, vec![Value::String(token.to_string())])
    }

    pub fn send_command(command: &str) -> Self {
        Self::new(
            request::SEND_COMMAND,
            vec![Value::String(command.to_string())],
        )
    }

    pub fn set_state(action: &str) -> Self {
        Self::new(request::SET_STATE, vec![Value::String(action.to_string())])
    }
}
