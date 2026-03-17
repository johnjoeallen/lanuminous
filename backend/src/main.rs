use anyhow::Result;
use clap::Parser;
use lanuminous::cli::Cli;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("lanuminous=info")),
        )
        .with_target(false)
        .compact()
        .init();

    let cli = Cli::parse();
    cli.run().await
}
