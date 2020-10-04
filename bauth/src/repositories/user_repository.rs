use async_trait::async_trait;
use sqlx::postgres::PgQueryAs;
use sqlx::{query_as, PgConnection};
use uuid::Uuid;

use crate::domain::user::{User, UserStore};
use crate::errors::RepositoryError;

#[async_trait]
impl UserStore for PgConnection {
    async fn save_user(&mut self, user: &User) -> Result<User, RepositoryError> {
        let user = query_as(
            r#"
            INSERT INTO users (
                id,
                username,
                email,
                password,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
            "#,
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(self)
        .await?;
        Ok(user)
    }

    async fn get_user_by_id(&mut self, id: &Uuid) -> Result<User, RepositoryError> {
        let user = query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(self)
            .await?;
        Ok(user)
    }

    async fn get_user_by_email(&mut self, email: &str) -> Result<User, RepositoryError> {
        let user = query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
}
