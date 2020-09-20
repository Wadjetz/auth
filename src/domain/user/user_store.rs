use async_trait::async_trait;

use crate::domain::user::User;
use crate::errors::RepositoryError;

#[async_trait]
pub trait UserStore {
    async fn save_user(&mut self, user: &User) -> Result<User, RepositoryError>;

    async fn get_user_by_email(&mut self, email: &str) -> Result<User, RepositoryError>;
}
