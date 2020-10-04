use chrono::{NaiveDateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AuthorizationAttempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub client_id: String,
    pub response_type: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub created_at: NaiveDateTime,
}

impl AuthorizationAttempt {
    pub fn new(
        user_id: Uuid,
        client_id: String,
        response_type: String,
        redirect_uri: String,
        scope: Option<String>,
        state: Option<String>,
    ) -> Self {
        let code: String = thread_rng().sample_iter(Alphanumeric).take(20).collect();
        Self {
            id: Uuid::new_v4(),
            user_id,
            code,
            client_id,
            response_type,
            redirect_uri,
            scope,
            state,
            created_at: Utc::now().naive_utc(),
        }
    }
}
