use actix_web::{
    get, post,
    web::{Data, Form, HttpResponse, Query},
};
use bauth::domain::application::ApplicationStore;
use bauth::domain::oauth::{
    AuthorizationAttempt, AuthorizationAttemptStore, AuthorizationRequest, AuthorizationResponse,
};
use bauth::domain::user::UserStore;
use bauth::password;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tera::{Context, Tera};

use crate::errors::ApiError;
use crate::utils::redirect_response;

#[get("/signup")]
pub async fn signup_form_route(
    pool: Data<PgPool>,
    template: Data<Tera>,
    authorization_request: Query<AuthorizationRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;

    let application = connection
        .get_application(&authorization_request.client_id)
        .await?;

    let mut ctx = Context::new();
    ctx.insert("app_name", &application.name);

    let querystring = authorization_request.to_querystring();
    ctx.insert("querystring", &querystring);

    let html = template.render("signup.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[post("/signup")]
pub async fn signup_route(
    pool: Data<PgPool>,
    signup_request: Form<SignupRequest>,
    authorization_request: Query<AuthorizationRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;

    let application = connection
        .get_application(&authorization_request.client_id)
        .await?;

    let user = password::secure_user(
        signup_request.username.clone(),
        signup_request.email.clone(),
        signup_request.password.clone(),
    )?;

    let user = connection.save_user(&user).await?;

    let authorization_attempt = AuthorizationAttempt::new(
        user.id,
        authorization_request.client_id.clone(),
        authorization_request.response_type.to_string(),
        authorization_request.redirect_uri.to_string(),
        authorization_request.scope.clone(),
        authorization_request.state.clone(),
    );

    let authorization_attempt = connection
        .save_authorization_attempt(&authorization_attempt)
        .await?;

    let authorization_response = AuthorizationResponse::from(authorization_attempt);

    let url = authorization_response.redirect_uri(&application.redirect_uri)?;

    dbg!(&url);

    Ok(redirect_response(&url))
}
