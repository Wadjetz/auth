use actix_web::{
    get,
    web::{Data, HttpResponse},
};
use bauth::domain::application::ApplicationStore;
use sqlx::PgPool;
use tera::{Context, Tera};

use crate::config::Config;
use crate::errors::ApiError;

#[get("/admin{tail:.*}")]
pub async fn admin_route(
    pool: Data<PgPool>,
    config: Data<Config>,
    template: Data<Tera>,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;
    let config = config.into_inner();
    let application = connection
        .get_application_by_name(&config.admin_app_name())
        .await?
        .ok_or(ApiError::NotFound)?;
    let mut ctx = Context::new();
    ctx.insert("client_id", &application.client_id);
    let html = template.render("admin.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
