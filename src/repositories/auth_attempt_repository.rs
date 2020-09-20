use async_trait::async_trait;
use sqlx::postgres::PgQueryAs;
use sqlx::{query_as, PgConnection};

use crate::domain::oauth::{AuthorizationAttempt, AuthorizationAttemptStore};
use crate::errors::RepositoryError;

#[async_trait]
impl AuthorizationAttemptStore for PgConnection {
    async fn save_authorization_attempt(
        &mut self,
        authorization_attempt: &AuthorizationAttempt,
    ) -> Result<AuthorizationAttempt, RepositoryError> {
        let application = query_as(
            r#"
            INSERT INTO authorization_attempts (
                id,
                user_id,
                code,
                client_id,
                response_type,
                redirect_uri,
                scope,
                state,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *
            "#,
        )
        .bind(&authorization_attempt.id)
        .bind(&authorization_attempt.user_id)
        .bind(&authorization_attempt.code)
        .bind(&authorization_attempt.client_id)
        .bind(&authorization_attempt.response_type)
        .bind(&authorization_attempt.redirect_uri)
        .bind(&authorization_attempt.scope)
        .bind(&authorization_attempt.state)
        .bind(authorization_attempt.created_at)
        .fetch_one(self)
        .await?;
        Ok(application)
    }

    async fn get_authorization_attempt(
        &mut self,
        id: &uuid::Uuid,
    ) -> Result<AuthorizationAttempt, RepositoryError> {
        let authorization_attempt = query_as("SELECT * FROM authorization_attempts WHERE id = $1")
            .bind(id)
            .fetch_one(self)
            .await?;
        Ok(authorization_attempt)
    }
}
