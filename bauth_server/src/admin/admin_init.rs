use bauth::domain::application::{Application, ApplicationStore, CreateApplication};
use sqlx::PgPool;

use crate::config::Config;
use crate::errors::ApiError;

pub async fn init_admin_application(
    pool: PgPool,
    config: &Config,
) -> Result<Application, ApiError> {
    let mut connection = pool.acquire().await?;
    let admin_app_name = config.admin_app_name();
    let maybe_admin_application = connection.get_application_by_name(&admin_app_name).await?;
    match maybe_admin_application {
        Some(application) => Ok(application),
        None => {
            let admin_redirect_uri = format!("{}/admin/callback", &config.base_uri);
            let create_application = CreateApplication::new(admin_app_name, admin_redirect_uri);
            let admin_application = Application::from(create_application);
            let admin_application = connection.save_application(&admin_application).await?;
            Ok(admin_application)
        }
    }
}
