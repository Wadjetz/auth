use actix_web::{http, HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

use crate::domain::oauth::OauthError;
use crate::utils::redirect_response;

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
    #[error("url")]
    Url(#[from] url::ParseError),
    #[error("repository")]
    Repository(#[from] RepositoryError),
    #[error("template")]
    Template(#[from] tera::Error),
    #[error("bcrypt")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("token")]
    Token(#[from] jsonwebtoken::errors::Error),
    #[error("token")]
    Oauth(#[from] OauthError),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        dbg!(self);
        match self {
            Self::Invalid { message } => HttpResponse::build(http::StatusCode::BAD_REQUEST)
                .json(json!({ "message": message })),
            Self::Duplicate | Self::Repository(RepositoryError::Duplicate) => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST)
                    .json(json!({ "message": "already.exists" }))
            }
            Self::NotFound | Self::Repository(RepositoryError::NotFound) => {
                HttpResponse::build(http::StatusCode::NOT_FOUND)
                    .json(json!({ "message": "not.found" }))
            }
            Self::Oauth(error) => redirect_response(&error.to_redirect_url()),
            Self::InternalServer
            | Self::IO(_)
            | Self::Sqlx(_)
            | Self::Repository(_)
            | Self::Template(_)
            | Self::Bcrypt(_)
            | Self::Token(_)
            | Self::Url(_) => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                .json(json!({ "message": "technical.error" })),
        }
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("already exist")]
    Duplicate,
    #[error("not found")]
    NotFound,
    #[error("database")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match &error {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(e) if e.code() == Some("23505") => Self::Duplicate,
            _ => Self::Database(error),
        }
    }
}
