use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("already exist")]
    Duplicate,
    #[error("not found")]
    NotFound,
    #[error("database")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match &error {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(e) if e.code() == Some("23505") => Self::Duplicate,
            _ => Self::Database(error),
        }
    }
}
