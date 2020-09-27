use actix_web::{
    get,
    web::{Data, HttpResponse, Query},
};
use sqlx::PgPool;

use crate::config::Config;
use crate::domain::application::ApplicationStore;
use crate::domain::oauth::{AuthorizationRequest, OauthError, OauthErrorKind};
use crate::errors::{ApiError, RepositoryError};
use crate::utils::redirect_response;

#[get("/authorize")]
pub async fn authorize_form_route(
    pool: Data<PgPool>,
    config: Data<Config>,
    authorization_request: Query<AuthorizationRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;

    let application = connection
        .get_application(&authorization_request.client_id)
        .await
        .map_err(|error| match error {
            RepositoryError::NotFound => OauthError::new(
                OauthErrorKind::UnauthorizedClient,
                authorization_request.redirect_uri.to_string(),
                Some(String::from("Client not found")),
                None,
            ),
            error => OauthError::new(
                OauthErrorKind::ServerError,
                authorization_request.redirect_uri.to_string(),
                Some(error.to_string()),
                None,
            ),
        })?;

    dbg!(&application);

    let querystring = &authorization_request.to_querystring();
    let url = format!("{}/login?{}", config.base_uri, querystring);

    dbg!(&url);

    Ok(redirect_response(&url))
}
