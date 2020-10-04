use actix_web::{
    post,
    web::{Data, HttpResponse, Query},
};
use bauth::domain::application::ApplicationStore;
use bauth::domain::oauth::AuthorizationAttemptStore;
use bauth::domain::user::UserStore;
use bauth::jwt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::ApiError;

///POST https://api.authorization-server.com/token
/// grant_type=authorization_code&
/// code=AUTH_CODE_HERE&
/// redirect_uri=REDIRECT_URI&
/// client_id=CLIENT_ID&
/// code_verifier=VERIFIER_STRING
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub code_verifier: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

impl TokenResponse {
    pub fn new(access_token: String, token_type: String, expires_in: usize) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
        }
    }
}

#[post("/token")]
pub async fn token_route(
    pool: Data<PgPool>,
    token_request: Query<TokenRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;

    let application = connection.get_application(&token_request.client_id).await?;

    let authorization_attempt = connection
        .get_authorization_attempt_by_code(&token_request.code)
        .await?;

    // TODO check everything

    let user = connection
        .get_user_by_id(&authorization_attempt.user_id)
        .await?;

    let (access_token, claime) = jwt::create_token(user, &application.client_secret)?;

    let token_response = TokenResponse::new(access_token, String::from("Bearer"), claime.exp);

    Ok(HttpResponse::Ok().json(token_response))
}
