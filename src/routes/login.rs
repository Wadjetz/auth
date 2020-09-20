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
use crate::password::verify_password;

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
    ctx.insert("client_id", &authorization_request.client_id);
    ctx.insert("response_type", &authorization_request.response_type);
    ctx.insert("redirect_uri", &authorization_request.redirect_uri);
    ctx.insert("scope", &authorization_request.scope);
    ctx.insert("state", &authorization_request.state);
    ctx.insert("state", &authorization_request.state);

    let querystring = authorization_request.to_querystring();
    ctx.insert("querystring", &querystring);

    let html = template.render("login.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub client_id: String,
    pub response_type: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
}

#[post("/login")]
pub async fn login_route(
    pool: Data<PgPool>,
    login_request: Form<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;

    let application = connection.get_application(&login_request.client_id).await?;

    let user = connection.get_user_by_email(&login_request.email).await?;

    if verify_password(&login_request.password, &user.password)? {
        let authorization_attempt = AuthorizationAttempt::new(
            user.id,
            login_request.client_id.clone(),
            login_request.response_type.clone(),
            login_request.redirect_uri.clone(),
            login_request.scope.clone(),
            login_request.state.clone(),
        );

        let authorization_attempt = connection
            .save_authorization_attempt(&authorization_attempt)
            .await?;

        let authorization_response = AuthorizationResponse::from(authorization_attempt);

        let url = authorization_response.redirect_uri(&application.redirect_uri)?;

        Ok(HttpResponse::Found()
            .header(header::LOCATION, url.as_str())
            .finish())
    } else {
        // TODO better message error
        Ok(HttpResponse::Unauthorized().finish())
    }
}
