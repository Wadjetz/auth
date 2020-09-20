use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

use crate::domain::user::User;
use crate::errors::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claime {
    pub sub: Uuid,
    pub exp: usize,
    pub user: User,
}

impl Claime {
    pub fn new(sub: Uuid, exp: usize, user: User) -> Self {
        Self { sub, exp, user }
    }
}

pub fn create_token(user: User, client_secret: &str) -> Result<(String, Claime), ApiError> {
    let claims = Claime::new(
        user.id,
        Utc::now().add(Duration::days(30)).timestamp() as usize,
        user,
    );
    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(client_secret.as_bytes()),
    )?;
    Ok((token, claims))
}

#[allow(dead_code)]
pub fn decode_token(token: &str, client_secret: &str) -> Result<Claime, ApiError> {
    let validation = Validation {
        validate_exp: true,
        algorithms: vec![Algorithm::HS512],
        ..Validation::default()
    };
    let claims = decode::<Claime>(
        token,
        &DecodingKey::from_secret(client_secret.as_bytes()),
        &validation,
    )?;
    let claims = claims.claims;
    Ok(claims)
}
