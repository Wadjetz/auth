use actix_web::{
    get,
    http::header,
    web::{Data, HttpResponse, Query},
};
use sqlx::PgPool;

use crate::domain::application::ApplicationStore;
use crate::domain::oauth::{AuthorizationRequest, AuthorizationResponse};
use crate::errors::ApiError;

#[get("/authorize")]
pub async fn authorize_form_route(
    pool: Data<PgPool>,
    authorization_request: Query<AuthorizationRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;

    let application = connection
        .get_application(&authorization_request.client_id)
        .await?;

    dbg!(&application);

    let querystring = &authorization_request.to_querystring();
    let url = format!("/login?{}", querystring);

    dbg!(&url);

    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

// TODO test route, remove it
#[get("/callback")]
pub async fn authorize_callback_route(
    authorization_response: Query<AuthorizationResponse>,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body(format!("{:?}", authorization_response)))
}
