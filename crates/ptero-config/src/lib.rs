//! Local config: `~/.config/ptero/config.toml`

use directories::ProjectDirs;
use ptero_protocol::{PteroError, PteroResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub panel_url: String,
    #[serde(default)]
    pub client_api_key: String,
    #[serde(default)]
    pub application_api_key: String,
    /// Node daemon token in `id.secret` form for Remote API.
    #[serde(default)]
    pub daemon_token: String,
}

/// Optional overrides from CLI flags / environment.
#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    pub panel_url: Option<String>,
    pub client_api_key: Option<String>,
    pub application_api_key: Option<String>,
    pub daemon_token: Option<String>,
}

impl Config {
    pub fn is_ready_client(&self) -> bool {
        !self.panel_url.is_empty() && !self.client_api_key.is_empty()
    }

    pub fn is_ready_application(&self) -> bool {
        !self.panel_url.is_empty() && !self.application_api_key.is_empty()
    }

    pub fn is_ready_remote(&self) -> bool {
        !self.panel_url.is_empty() && is_daemon_token_format(&self.daemon_token)
    }

    /// Any surface that can talk to the panel (url + at least one credential).
    pub fn is_ready(&self) -> bool {
        self.is_ready_client() || self.is_ready_application() || self.is_ready_remote()
    }

    pub fn require_url(&self) -> PteroResult<&str> {
        if self.panel_url.is_empty() {
            Err(PteroError::Config(
                "panel_url not set; run `ptero config set-url <url>` or pass --url".into(),
            ))
        } else {
            Ok(&self.panel_url)
        }
    }

    pub fn require_client_key(&self) -> PteroResult<&str> {
        if self.client_api_key.is_empty() {
            Err(PteroError::Config(
                "client_api_key not set; run `ptero config set-client-key <key>` or pass --client-key"
                    .into(),
            ))
        } else {
            Ok(&self.client_api_key)
        }
    }

    pub fn require_application_key(&self) -> PteroResult<&str> {
        if self.application_api_key.is_empty() {
            Err(PteroError::Config(
                "application_api_key not set; run `ptero config set-app-key <key>` or pass --app-key"
                    .into(),
            ))
        } else {
            Ok(&self.application_api_key)
        }
    }

    pub fn require_daemon_token(&self) -> PteroResult<&str> {
        if !is_daemon_token_format(&self.daemon_token) {
            Err(PteroError::Config(
                "daemon_token not set or invalid; expected `id.secret` (Node daemon token); run `ptero config set-daemon-token` or pass --daemon-token"
                    .into(),
            ))
        } else {
            Ok(&self.daemon_token)
        }
    }

    pub fn apply_overrides(&mut self, o: &ConfigOverrides) {
        if let Some(v) = &o.panel_url {
            if !v.is_empty() {
                self.panel_url = v.clone();
            }
        }
        if let Some(v) = &o.client_api_key {
            if !v.is_empty() {
                self.client_api_key = v.clone();
            }
        }
        if let Some(v) = &o.application_api_key {
            if !v.is_empty() {
                self.application_api_key = v.clone();
            }
        }
        if let Some(v) = &o.daemon_token {
            if !v.is_empty() {
                self.daemon_token = v.clone();
            }
        }
    }

    /// Apply well-known environment variables if fields are still empty.
    pub fn apply_env(&mut self) {
        apply_env_if_empty(&mut self.panel_url, "PTERO_URL");
        apply_env_if_empty(&mut self.client_api_key, "PTERO_CLIENT_KEY");
        apply_env_if_empty(&mut self.application_api_key, "PTERO_APP_KEY");
        apply_env_if_empty(&mut self.daemon_token, "PTERO_DAEMON_TOKEN");
    }
}

fn apply_env_if_empty(field: &mut String, key: &str) {
    if field.is_empty() {
        if let Ok(v) = std::env::var(key) {
            if !v.is_empty() {
                *field = v;
            }
        }
    }
}

/// Daemon token must be `id.secret` (at least one dot, both sides non-empty).
pub fn is_daemon_token_format(token: &str) -> bool {
    match token.split_once('.') {
        Some((id, secret)) => !id.is_empty() && !secret.is_empty(),
        None => false,
    }
}

pub fn config_dir() -> PteroResult<PathBuf> {
    let dirs = ProjectDirs::from("com", "Pterodactyl", "ptero")
        .ok_or_else(|| PteroError::Config("cannot resolve config directory".into()))?;
    Ok(dirs.config_dir().to_path_buf())
}

pub fn config_path() -> PteroResult<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn load() -> PteroResult<Config> {
    let path = config_path()?;
    load_from(&path)
}

pub fn load_from(path: &Path) -> PteroResult<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = fs::read_to_string(path)?;
    toml::from_str(&text).map_err(|e| PteroError::Config(e.to_string()))
}

/// Load config file, then apply env, then CLI overrides.
pub fn load_with_overrides(
    path: Option<&Path>,
    overrides: &ConfigOverrides,
) -> PteroResult<Config> {
    let mut cfg = match path {
        Some(p) => load_from(p)?,
        None => load()?,
    };
    cfg.apply_env();
    cfg.apply_overrides(overrides);
    Ok(cfg)
}

pub fn save(cfg: &Config) -> PteroResult<()> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    let path = dir.join("config.toml");
    save_to(&path, cfg)
}

pub fn save_to(path: &Path, cfg: &Config) -> PteroResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = toml::to_string_pretty(cfg).map_err(|e| PteroError::Config(e.to_string()))?;
    fs::write(path, text)?;
    Ok(())
}
