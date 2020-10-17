use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::application::CreateApplication;
use crate::security::generate_random_string;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Application {
    pub id: Uuid,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Application {
    pub fn from(create_application: CreateApplication) -> Self {
        let client_id: String = generate_random_string(30);
        let client_secret: String = generate_random_string(50);
        Self {
            id: Uuid::new_v4(),
            client_id,
            client_secret,
            name: create_application.name,
            description: create_application.description,
            website_url: create_application.website_url,
            redirect_uri: create_application.redirect_uri,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
