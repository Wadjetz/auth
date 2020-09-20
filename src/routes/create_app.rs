use actix_web::{
    post,
    web::{Data, HttpResponse, Json},
};
use sqlx::PgPool;

use crate::domain::application::{Application, ApplicationStore, CreateApplication};
use crate::errors::ApiError;

#[post("/api/application")]
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
