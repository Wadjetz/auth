use actix_web::{http, HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServer,
    #[error("invalid")]
    Invalid { message: String },
    #[error("duplicate")]
    Duplicate,
    #[error("NotFound")]
    NotFound,
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("sqlx")]
    Sqlx(#[from] sqlx::Error),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        dbg!(self);
        match self {
            Self::Invalid { message } => HttpResponse::build(http::StatusCode::BAD_REQUEST)
                .json(json!({ "message": message })),
            Self::Duplicate => HttpResponse::build(http::StatusCode::BAD_REQUEST)
                .json(json!({ "message": "already.exists" })),
            Self::NotFound => HttpResponse::build(http::StatusCode::NOT_FOUND)
                .json(json!({ "message": "not.found" })),
            Self::InternalServer | Self::IO(_) | Self::Sqlx(_) => {
                HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({ "message": "technical.error" }))
            }
        }
    }
}
