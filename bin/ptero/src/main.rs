use anyhow::Result;
use clap::Parser;
use ptero_cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    // No args (only binary name) → TUI. Any args → CLI (including --help).
    if std::env::args_os().len() <= 1 {
        return ptero_tui::run().await;
    }

    let cli = Cli::parse();
    if cli.command.is_none() {
        // Flags only without subcommand → still prefer TUI with overrides applied later.
        return ptero_tui::run().await;
    }
    ptero_cli::run(cli).await
}
