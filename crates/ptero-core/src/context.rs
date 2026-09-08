use ptero_config::Config;
use ptero_panel::HttpPanelClient;
use ptero_protocol::{PteroError, PteroResult};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub panel: Arc<HttpPanelClient>,
    pub config: Config,
}

pub struct AppContextBuilder {
    config: Config,
}

impl AppContextBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn build(self) -> PteroResult<AppContext> {
        let url = self.config.require_url()?;
        let mut client = HttpPanelClient::new(url)?;
        if !self.config.client_api_key.is_empty() {
            client = client.with_client_api_key(&self.config.client_api_key);
        }
        if !self.config.application_api_key.is_empty() {
            client = client.with_application_api_key(&self.config.application_api_key);
        }
        if !self.config.daemon_token.is_empty() {
            client = client.with_daemon_token(&self.config.daemon_token);
        }
        Ok(AppContext {
            panel: Arc::new(client),
            config: self.config,
        })
    }

    /// Build with URL only (credentials optional). Useful for TUI home / config screens.
    pub fn build_unauthenticated(self) -> PteroResult<AppContext> {
        if self.config.panel_url.is_empty() {
            return Err(PteroError::Config(
                "panel_url not set; run `ptero config set-url <url>`".into(),
            ));
        }
        let mut client = HttpPanelClient::new(self.config.panel_url.as_str())?;
        if !self.config.client_api_key.is_empty() {
            client = client.with_client_api_key(&self.config.client_api_key);
        }
        if !self.config.application_api_key.is_empty() {
            client = client.with_application_api_key(&self.config.application_api_key);
        }
        if !self.config.daemon_token.is_empty() {
            client = client.with_daemon_token(&self.config.daemon_token);
        }
        Ok(AppContext {
            panel: Arc::new(client),
            config: self.config,
        })
    }
}

impl AppContext {
    pub fn from_config(config: Config) -> PteroResult<Self> {
        AppContextBuilder::new(config).build()
    }

    pub fn require_client_ready(&self) -> PteroResult<()> {
        if self.config.is_ready_client() {
            Ok(())
        } else {
            self.config.require_client_key().map(|_| ())
        }
    }

    pub fn require_application_ready(&self) -> PteroResult<()> {
        if self.config.is_ready_application() {
            Ok(())
        } else {
            self.config.require_application_key().map(|_| ())
        }
    }

    pub fn require_remote_ready(&self) -> PteroResult<()> {
        if self.config.is_ready_remote() {
            Ok(())
        } else {
            self.config.require_daemon_token().map(|_| ())
        }
    }
}
