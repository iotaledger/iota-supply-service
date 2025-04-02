// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::net::SocketAddr;

use axum::{Extension, Json, Router, http, response::IntoResponse, routing::get};
use http::Method;
use iota_sdk::{IotaClient, IotaClientBuilder};
use serde::Serialize;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

const NANOS_PER_IOTA: u64 = 1_000_000_000;

use crate::errors::ApiError;

pub(crate) fn spawn_rest_server(
    socket_addr: SocketAddr,
    cancel_token: CancellationToken,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let app = build_app().await;

        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .expect("failed to bind to socket");

        info!("Listening on: {}", socket_addr);

        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                cancel_token.cancelled().await;
                info!("Shutdown signal received.");
            })
            .await
            .inspect_err(|e| error!("Server encountered an error: {e}"))
            .ok();
    })
}

async fn build_app() -> Router {
    let iota_client = IotaClientBuilder::default().build_mainnet().await.unwrap();

    // Allow all origins (CORS policy) - This is safe because the API is public and
    // does not require authentication. CORS is a browser-enforced mechanism
    // that restricts cross-origin requests, but since the API is already accessible
    // without credentials or sensitive data, there is no additional security risk.
    // Abuse should be mitigated via backend protections such as rate-limiting.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Method::GET)
        .allow_headers(Any);

    Router::new()
        .route("/supply/circulating", get(circulating_supply))
        .route("/supply/total", get(total_supply))
        .layer(Extension(iota_client))
        .layer(cors)
        .fallback(fallback)
}

async fn fallback() -> impl IntoResponse {
    ApiError::Forbidden
}

#[derive(Serialize)]
struct SupplyResponse(u64);

/// Provides the circulating supply.
async fn circulating_supply(
    Extension(client): Extension<IotaClient>,
) -> Result<impl IntoResponse, ApiError> {
    let circulating_supply = client.coin_read_api().get_circulating_supply().await?;
    let iotas = circulating_supply.value / NANOS_PER_IOTA;
    Ok(Json(SupplyResponse(iotas)))
}

/// Provides the total supply.
async fn total_supply(
    Extension(client): Extension<IotaClient>,
) -> Result<impl IntoResponse, ApiError> {
    let total_supply = client
        .coin_read_api()
        .get_total_supply("0x2::iota::IOTA")
        .await?;
    let iotas = total_supply.value / NANOS_PER_IOTA;
    Ok(Json(SupplyResponse(iotas)))
}
