use ptero_protocol::{PteroError, PteroResult};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

/// Which credential / path family a request uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiSurface {
    Client,
    Application,
    Remote,
}

#[derive(Clone)]
pub struct HttpPanelClient {
    base: String,
    client_api_key: Option<String>,
    application_api_key: Option<String>,
    daemon_token: Option<String>,
    http: Client,
}

impl HttpPanelClient {
    pub fn new(panel_url: impl Into<String>) -> PteroResult<Self> {
        let base = panel_url.into().trim_end_matches('/').to_string();
        if base.is_empty() {
            return Err(PteroError::Config("panel_url is empty".into()));
        }
        let http = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent(concat!("ptero-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| PteroError::Network(e.to_string()))?;
        Ok(Self {
            base,
            client_api_key: None,
            application_api_key: None,
            daemon_token: None,
            http,
        })
    }

    pub fn with_client_api_key(mut self, key: impl Into<String>) -> Self {
        let k = key.into();
        if !k.is_empty() {
            self.client_api_key = Some(k);
        }
        self
    }

    pub fn with_application_api_key(mut self, key: impl Into<String>) -> Self {
        let k = key.into();
        if !k.is_empty() {
            self.application_api_key = Some(k);
        }
        self
    }

    pub fn with_daemon_token(mut self, token: impl Into<String>) -> Self {
        let t = token.into();
        if !t.is_empty() {
            self.daemon_token = Some(t);
        }
        self
    }

    pub fn set_client_api_key(&mut self, key: Option<String>) {
        self.client_api_key = key.filter(|k| !k.is_empty());
    }

    pub fn set_application_api_key(&mut self, key: Option<String>) {
        self.application_api_key = key.filter(|k| !k.is_empty());
    }

    pub fn set_daemon_token(&mut self, token: Option<String>) {
        self.daemon_token = token.filter(|t| !t.is_empty());
    }

    fn url(&self, path: &str) -> String {
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };
        format!("{}{path}", self.base)
    }

    fn bearer_for(&self, surface: ApiSurface) -> PteroResult<&str> {
        match surface {
            ApiSurface::Client => self.client_api_key.as_deref().ok_or_else(|| {
                PteroError::Config(
                    "client_api_key required for Client API; set config or --client-key".into(),
                )
            }),
            ApiSurface::Application => self.application_api_key.as_deref().ok_or_else(|| {
                PteroError::Config(
                    "application_api_key required for Application API; set config or --app-key"
                        .into(),
                )
            }),
            ApiSurface::Remote => self.daemon_token.as_deref().ok_or_else(|| {
                PteroError::Config(
                    "daemon_token required for Remote API; set config or --daemon-token (id.secret)"
                        .into(),
                )
            }),
        }
    }

    fn apply_auth_headers(
        &self,
        surface: ApiSurface,
        rb: RequestBuilder,
        json_content_type: bool,
    ) -> PteroResult<RequestBuilder> {
        let token = self.bearer_for(surface)?;
        let mut rb = rb
            .header("Accept", "Application/vnd.pterodactyl.v1+json")
            .header("Authorization", format!("Bearer {token}"));
        if json_content_type {
            rb = rb.header("Content-Type", "application/json");
        }
        Ok(rb)
    }

    fn parse_method(method: &str) -> PteroResult<Method> {
        match method.to_ascii_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            other => Err(PteroError::Message(format!(
                "unsupported HTTP method: {other}"
            ))),
        }
    }

    /// Low-level JSON request. Empty 2xx → `Null`.
    pub async fn request_raw(
        &self,
        surface: ApiSurface,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<Value> {
        let text = self
            .request_text(surface, method, path, query, body, true)
            .await?;
        if text.is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_str(&text)
            .map_err(|e| PteroError::Serde(format!("response parse failed: {e}; body={text}")))
    }

    async fn request_text(
        &self,
        surface: ApiSurface,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
        json_content_type: bool,
    ) -> PteroResult<String> {
        let method = Self::parse_method(method)?;
        let mut rb = self.apply_auth_headers(
            surface,
            self.http.request(method, self.url(path)),
            json_content_type,
        )?;
        if !query.is_empty() {
            rb = rb.query(query);
        }
        if let Some(b) = body {
            rb = rb.json(b);
        }
        let resp = rb
            .send()
            .await
            .map_err(|e| PteroError::Network(e.to_string()))?;
        let status = resp.status().as_u16() as i32;
        let text = resp
            .text()
            .await
            .map_err(|e| PteroError::Network(e.to_string()))?;
        if !(200..300).contains(&status) {
            return Err(PteroError::from_api_body(status, &text));
        }
        Ok(text)
    }

    /// GET/POST/etc returning typed JSON.
    pub async fn request_json<T: DeserializeOwned>(
        &self,
        surface: ApiSurface,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<T> {
        let text = self
            .request_text(surface, method, path, query, body, true)
            .await?;
        if text.is_empty() {
            return Err(PteroError::Serde(
                "expected JSON body but response was empty".into(),
            ));
        }
        serde_json::from_str(&text)
            .map_err(|e| PteroError::Serde(format!("response parse failed: {e}; body={text}")))
    }

    /// 2xx with empty or ignored body.
    pub async fn request_empty(
        &self,
        surface: ApiSurface,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<()> {
        let _ = self
            .request_text(surface, method, path, query, body, true)
            .await?;
        Ok(())
    }

    pub async fn get_json<T: DeserializeOwned>(
        &self,
        surface: ApiSurface,
        path: &str,
        query: &[(String, String)],
    ) -> PteroResult<T> {
        self.request_json(surface, "GET", path, query, None).await
    }

    pub async fn post_json_body<B: Serialize, T: DeserializeOwned>(
        &self,
        surface: ApiSurface,
        path: &str,
        body: &B,
    ) -> PteroResult<T> {
        let v = serde_json::to_value(body)?;
        self.request_json(surface, "POST", path, &[], Some(&v))
            .await
    }

    pub async fn post_empty_body<B: Serialize>(
        &self,
        surface: ApiSurface,
        path: &str,
        body: &B,
    ) -> PteroResult<()> {
        let v = serde_json::to_value(body)?;
        self.request_empty(surface, "POST", path, &[], Some(&v))
            .await
    }

    pub async fn put_json_body<B: Serialize, T: DeserializeOwned>(
        &self,
        surface: ApiSurface,
        path: &str,
        body: &B,
    ) -> PteroResult<T> {
        let v = serde_json::to_value(body)?;
        self.request_json(surface, "PUT", path, &[], Some(&v)).await
    }

    pub async fn put_empty_body<B: Serialize>(
        &self,
        surface: ApiSurface,
        path: &str,
        body: &B,
    ) -> PteroResult<()> {
        let v = serde_json::to_value(body)?;
        self.request_empty(surface, "PUT", path, &[], Some(&v))
            .await
    }

    pub async fn patch_json_body<B: Serialize, T: DeserializeOwned>(
        &self,
        surface: ApiSurface,
        path: &str,
        body: &B,
    ) -> PteroResult<T> {
        let v = serde_json::to_value(body)?;
        self.request_json(surface, "PATCH", path, &[], Some(&v))
            .await
    }

    pub async fn delete_empty(&self, surface: ApiSurface, path: &str) -> PteroResult<()> {
        self.request_empty(surface, "DELETE", path, &[], None).await
    }

    /// File contents: text/plain response.
    pub async fn get_plain(
        &self,
        surface: ApiSurface,
        path: &str,
        query: &[(String, String)],
    ) -> PteroResult<String> {
        self.request_text(surface, "GET", path, query, None, true)
            .await
    }

    /// File write: raw body bytes with `?file=` query (Panel expects request body as content).
    pub async fn post_raw_body(
        &self,
        surface: ApiSurface,
        path: &str,
        query: &[(String, String)],
        body: &str,
    ) -> PteroResult<()> {
        let method = Method::POST;
        let mut rb =
            self.apply_auth_headers(surface, self.http.request(method, self.url(path)), false)?;
        rb = rb
            .header("Content-Type", "text/plain")
            .query(query)
            .body(body.to_string());
        let resp = rb
            .send()
            .await
            .map_err(|e| PteroError::Network(e.to_string()))?;
        let status = resp.status().as_u16() as i32;
        let text = resp
            .text()
            .await
            .map_err(|e| PteroError::Network(e.to_string()))?;
        if !(200..300).contains(&status) {
            return Err(PteroError::from_api_body(status, &text));
        }
        Ok(())
    }
}
