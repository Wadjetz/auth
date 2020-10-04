use actix_web::{
    get, post,
    web::{Data, HttpResponse, Json},
};
use sqlx::PgPool;

use crate::domain::application::{Application, ApplicationStore, CreateApplication};
use crate::errors::ApiError;

#[get("/api/applications")]
pub async fn get_applications_route(pool: Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;
    let applications = connection.get_applications().await?;
    Ok(HttpResponse::Ok().json(applications))
}

#[post("/api/applications")]
pub async fn create_application_route(
    pool: Data<PgPool>,
    payload: Json<CreateApplication>,
) -> Result<HttpResponse, ApiError> {
    // TODO validations
    let mut connection = pool.acquire().await?;
    let application = Application::from(payload.into_inner());
    let application = connection.save_application(&application).await?;
    Ok(HttpResponse::Ok().json(application))
}
