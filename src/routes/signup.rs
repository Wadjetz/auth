use actix_web::{
    get,
    http::header,
    post,
    web::{Data, Form, HttpResponse, Query},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tera::{Context, Tera};

use crate::domain::application::ApplicationStore;
use crate::domain::oauth::{
    AuthorizationAttempt, AuthorizationAttemptStore, AuthorizationRequest, AuthorizationResponse,
};
use crate::domain::user::UserStore;
use crate::errors::ApiError;
use crate::password;

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
        authorization_request.response_type.to_string().clone(),
        authorization_request.redirect_uri.clone(),
        authorization_request.scope.clone(),
        authorization_request.state.clone(),
    );

    let authorization_attempt = connection
        .save_authorization_attempt(&authorization_attempt)
        .await?;

    let authorization_response = AuthorizationResponse::from(authorization_attempt);

    let url = authorization_response.redirect_uri(&application.redirect_uri)?;

    dbg!(&url);

    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}
