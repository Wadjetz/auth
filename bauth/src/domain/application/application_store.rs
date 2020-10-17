use async_trait::async_trait;

use crate::domain::application::Application;
use crate::errors::RepositoryError;

#[async_trait]
pub trait ApplicationStore {
    async fn save_application(
        &mut self,
        application: &Application,
    ) -> Result<Application, RepositoryError>;

    async fn get_application(&mut self, client_id: &str) -> Result<Application, RepositoryError>;

    async fn get_application_by_name(
        &mut self,
        name: &str,
    ) -> Result<Option<Application>, RepositoryError>;

    async fn get_applications(&mut self) -> Result<Vec<Application>, RepositoryError>;
}
