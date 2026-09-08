//! Pterodactyl TUI (ratatui).

mod app;
mod ui;

pub use app::run_app;

use anyhow::Result;
use ptero_config::Config;
use ptero_core::AppContext;

pub async fn run() -> Result<()> {
    let cfg = ptero_config::load().unwrap_or_default();
    run_with_config(cfg).await
}

pub async fn run_with_config(cfg: Config) -> Result<()> {
    let ctx = AppContext::from_config(cfg.clone()).ok();
    run_app(ctx, cfg).await
}
