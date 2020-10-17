use async_trait::async_trait;
use sqlx::postgres::PgQueryAs;
use sqlx::{query_as, PgConnection};

use crate::domain::application::{Application, ApplicationStore};
use crate::errors::RepositoryError;

#[async_trait]
impl ApplicationStore for PgConnection {
    async fn save_application(
        &mut self,
        application: &Application,
    ) -> Result<Application, RepositoryError> {
        let application = query_as(
            r#"
            INSERT INTO applications (
                id,
                client_id,
                client_secret,
                redirect_uri,
                name,
                description,
                website_url,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *
            "#,
        )
        .bind(application.id)
        .bind(&application.client_id)
        .bind(&application.client_secret)
        .bind(&application.redirect_uri)
        .bind(&application.name)
        .bind(&application.description)
        .bind(&application.website_url)
        .bind(application.created_at)
        .bind(application.updated_at)
        .fetch_one(self)
        .await?;
        Ok(application)
    }

    async fn get_application(&mut self, client_id: &str) -> Result<Application, RepositoryError> {
        let application = query_as("SELECT * FROM applications WHERE client_id = $1")
            .bind(client_id)
            .fetch_one(self)
            .await?;
        Ok(application)
    }

    async fn get_applications(&mut self) -> Result<Vec<Application>, RepositoryError> {
        let applications = query_as("SELECT * FROM applications")
            .fetch_all(self)
            .await?;
        Ok(applications)
    }

    async fn get_application_by_name(
        &mut self,
        name: &str,
    ) -> Result<Option<Application>, RepositoryError> {
        let application = query_as("SELECT * FROM applications WHERE name = $1")
            .bind(name)
            .fetch_optional(self)
            .await?;
        Ok(application)
    }
}
