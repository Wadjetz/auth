use actix_web::{
    get, post,
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
use crate::password::verify_password;
use crate::utils::redirect_response;

#[get("/login")]
pub async fn login_form_route(
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

    let html = template.render("login.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login_route(
    pool: Data<PgPool>,
    login_request: Form<LoginRequest>,
    authorization_request: Query<AuthorizationRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;

    let application = connection
        .get_application(&authorization_request.client_id)
        .await?;

    let user = connection.get_user_by_email(&login_request.email).await?;

    if verify_password(&login_request.password, &user.password)? {
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
    } else {
        // TODO better message error
        Ok(HttpResponse::Unauthorized().finish())
    }
}
