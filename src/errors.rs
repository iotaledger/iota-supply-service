// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ApiError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("forbidden")]
    Forbidden,
}

impl From<iota_sdk::error::Error> for ApiError {
    fn from(error: iota_sdk::error::Error) -> Self {
        ApiError::ServiceUnavailable(error.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ApiError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
        };

        let body = Json(ErrorResponse {
            error_code: status_code.as_u16().to_string(),
            error_message: self.to_string(),
        });

        (status_code, body).into_response()
    }
}

/// Describes the response body of a unsuccessful HTTP request.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct ErrorResponse {
    error_code: String,
    error_message: String,
}
