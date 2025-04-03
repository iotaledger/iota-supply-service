// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use tokio::signal::unix::{SignalKind, signal};
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

mod errors;
mod server;

use tokio_util::sync::CancellationToken;

use crate::server::spawn_rest_server;

#[derive(Parser, Clone, Debug)]
#[clap(
    name = "IOTA supply REST API",
    about = "Provides the IOTA circulating and total supply."
)]
struct Cli {
    #[clap(long, default_value = "INFO", env = "LOG_LEVEL")]
    log_level: Level,
    #[clap(long, default_value = "0.0.0.0:4000", env = "REST_API_SOCKET_ADDRESS")]
    rest_api_address: std::net::SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    init_tracing(cli.log_level);

    // Set up a CTRL+C handler for graceful shutdown
    let token = setup_shutdown_signal();

    // Spawn the REST server
    spawn_rest_server(cli.rest_api_address, token)
        .await
        .inspect_err(|e| error!("REST server terminated with error: {e}"))??;

    Ok(())
}

/// Initialize the tracing with custom subscribers
fn init_tracing(log_level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

/// Set up a CTRL+C handler for graceful shutdown
fn setup_shutdown_signal() -> CancellationToken {
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    tokio::task::spawn(async move {
        let mut sigint = signal(SignalKind::interrupt()).expect("failed to listen for SIGINT");
        let mut sigterm = signal(SignalKind::terminate()).expect("failed to listen for SIGTERM");

        tokio::select! {
            _ = sigint.recv() => {
                info!("SIGINT received, shutting down.");
            }
            _ = sigterm.recv() => {
                info!("SIGTERM received, shutting down.");
            }
        }

        cloned_token.cancel();
    });

    token
}
