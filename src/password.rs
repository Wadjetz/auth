use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

use crate::domain::user::User;

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    Ok(verify(password, hashed_password)?)
}

pub fn secure_user(username: String, email: String, password: String) -> Result<User, BcryptError> {
    let hashed_password = hash_password(&password)?;
    let user = User::new(username, email, hashed_password);
    Ok(user)
}
