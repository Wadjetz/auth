use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::domain::application::CreateApplication;

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
        let client_id: String = thread_rng().sample_iter(Alphanumeric).take(30).collect();
        let client_secret: String = thread_rng().sample_iter(Alphanumeric).take(50).collect();
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
