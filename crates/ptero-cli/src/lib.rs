//! CLI command definitions and runners.

mod commands;
mod output;

pub use commands::{Cli, Command};

use anyhow::Result;
use ptero_config::{Config, ConfigOverrides};
use ptero_core::AppContext;

pub async fn run(cli: Cli) -> Result<()> {
    commands::dispatch(cli).await
}

pub fn load_config(cli: &Cli) -> Result<Config> {
    let overrides = ConfigOverrides {
        panel_url: cli.url.clone(),
        client_api_key: cli.client_key.clone(),
        application_api_key: cli.app_key.clone(),
        daemon_token: cli.daemon_token.clone(),
    };
    Ok(ptero_config::load_with_overrides(
        cli.config.as_deref(),
        &overrides,
    )?)
}

pub fn ctx_from_cli(cli: &Cli) -> Result<AppContext> {
    let cfg = load_config(cli)?;
    Ok(AppContext::from_config(cfg)?)
}
