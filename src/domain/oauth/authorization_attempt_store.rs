use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::oauth::AuthorizationAttempt;
use crate::errors::RepositoryError;

#[async_trait]
pub trait AuthorizationAttemptStore {
    async fn save_authorization_attempt(
        &mut self,
        authorization_attempt: &AuthorizationAttempt,
    ) -> Result<AuthorizationAttempt, RepositoryError>;

    async fn get_authorization_attempt(
        &mut self,
        id: &Uuid,
    ) -> Result<AuthorizationAttempt, RepositoryError>;
}
