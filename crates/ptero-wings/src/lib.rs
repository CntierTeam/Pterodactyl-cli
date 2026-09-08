//! Wings console WebSocket client.

use futures::{SinkExt, StreamExt};
use ptero_protocol::ws::{event, WsMessage};
use ptero_protocol::{PteroError, PteroResult, WebsocketCredentials};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// Events surfaced to CLI/TUI consumers.
#[derive(Debug, Clone)]
pub enum ConsoleEvent {
    AuthSuccess,
    ConsoleOutput(String),
    Status(String),
    Stats(Value),
    DaemonMessage(String),
    DaemonError(String),
    TokenExpiring,
    TokenExpired,
    Other(String, Vec<Value>),
    Closed,
}

/// Async callback used to refresh JWT when Wings signals expiry.
pub type TokenRefresh = Arc<dyn Fn() -> TokenRefreshFut + Send + Sync>;

pub type TokenRefreshFut =
    std::pin::Pin<Box<dyn std::future::Future<Output = PteroResult<WebsocketCredentials>> + Send>>;

pub struct WingsConsole {
    sink: Arc<
        Mutex<
            futures::stream::SplitSink<
                tokio_tungstenite::WebSocketStream<
                    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
                >,
                Message,
            >,
        >,
    >,
    events: mpsc::UnboundedReceiver<ConsoleEvent>,
    _pump: tokio::task::JoinHandle<()>,
}

impl WingsConsole {
    /// Connect to Wings socket, authenticate, and start event pump with token refresh.
    pub async fn connect(creds: WebsocketCredentials, refresh: TokenRefresh) -> PteroResult<Self> {
        let (ws, _) = connect_async(&creds.socket)
            .await
            .map_err(|e| PteroError::Wings(format!("connect failed: {e}")))?;
        let (mut sink, mut stream) = ws.split();

        let auth = WsMessage::auth(&creds.token);
        let auth_text =
            serde_json::to_string(&auth).map_err(|e| PteroError::Serde(e.to_string()))?;
        sink.send(Message::Text(auth_text.into()))
            .await
            .map_err(|e| PteroError::Wings(format!("auth send failed: {e}")))?;

        let sink = Arc::new(Mutex::new(sink));
        let sink_c = sink.clone();
        let (tx, rx) = mpsc::unbounded_channel();

        let pump = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                let Ok(msg) = msg else {
                    let _ = tx.send(ConsoleEvent::Closed);
                    break;
                };
                match msg {
                    Message::Text(text) => {
                        let parsed: Result<WsMessage, _> = serde_json::from_str(&text);
                        let Ok(wm) = parsed else {
                            continue;
                        };
                        match wm.event.as_str() {
                            e if e == event::AUTH_SUCCESS => {
                                let _ = tx.send(ConsoleEvent::AuthSuccess);
                            }
                            e if e == event::CONSOLE_OUTPUT => {
                                let line = wm
                                    .args
                                    .first()
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let _ = tx.send(ConsoleEvent::ConsoleOutput(line));
                            }
                            e if e == event::STATUS => {
                                let s = wm
                                    .args
                                    .first()
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let _ = tx.send(ConsoleEvent::Status(s));
                            }
                            e if e == event::STATS => {
                                let v = wm.args.first().cloned().unwrap_or(Value::Null);
                                let _ = tx.send(ConsoleEvent::Stats(v));
                            }
                            e if e == event::DAEMON_MESSAGE => {
                                let s = wm
                                    .args
                                    .first()
                                    .map(|v| match v {
                                        Value::String(s) => s.clone(),
                                        other => other.to_string(),
                                    })
                                    .unwrap_or_default();
                                let _ = tx.send(ConsoleEvent::DaemonMessage(s));
                            }
                            e if e == event::DAEMON_ERROR => {
                                let s = wm
                                    .args
                                    .first()
                                    .map(|v| match v {
                                        Value::String(s) => s.clone(),
                                        other => other.to_string(),
                                    })
                                    .unwrap_or_default();
                                let _ = tx.send(ConsoleEvent::DaemonError(s));
                            }
                            e if e == event::TOKEN_EXPIRING || e == event::TOKEN_EXPIRED => {
                                let is_expired = e == event::TOKEN_EXPIRED;
                                let _ = tx.send(if is_expired {
                                    ConsoleEvent::TokenExpired
                                } else {
                                    ConsoleEvent::TokenExpiring
                                });
                                match refresh().await {
                                    Ok(new_creds) => {
                                        let auth = WsMessage::auth(&new_creds.token);
                                        if let Ok(text) = serde_json::to_string(&auth) {
                                            let mut s = sink_c.lock().await;
                                            let _ = s.send(Message::Text(text.into())).await;
                                        }
                                    }
                                    Err(err) => {
                                        let _ = tx.send(ConsoleEvent::DaemonError(format!(
                                            "token refresh failed: {err}"
                                        )));
                                    }
                                }
                            }
                            _ => {
                                let _ = tx.send(ConsoleEvent::Other(wm.event, wm.args));
                            }
                        }
                    }
                    Message::Close(_) => {
                        let _ = tx.send(ConsoleEvent::Closed);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(Self {
            sink,
            events: rx,
            _pump: pump,
        })
    }

    pub async fn send_command(&self, command: &str) -> PteroResult<()> {
        self.send_msg(WsMessage::send_command(command)).await
    }

    pub async fn set_state(&self, action: &str) -> PteroResult<()> {
        self.send_msg(WsMessage::set_state(action)).await
    }

    pub async fn request_logs(&self) -> PteroResult<()> {
        self.send_msg(WsMessage::new(
            ptero_protocol::ws::request::SEND_LOGS,
            vec![],
        ))
        .await
    }

    async fn send_msg(&self, msg: WsMessage) -> PteroResult<()> {
        let text = serde_json::to_string(&msg).map_err(|e| PteroError::Serde(e.to_string()))?;
        let mut sink = self.sink.lock().await;
        sink.send(Message::Text(text.into()))
            .await
            .map_err(|e| PteroError::Wings(format!("send failed: {e}")))
    }

    pub async fn next(&mut self) -> Option<ConsoleEvent> {
        self.events.recv().await
    }

    pub async fn close(self) {
        let mut sink = self.sink.lock().await;
        let _ = sink.close().await;
    }
}
