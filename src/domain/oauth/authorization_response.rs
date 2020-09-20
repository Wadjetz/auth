use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

use crate::domain::oauth::AuthorizationAttempt;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationResponse {
    pub code: String,
    pub state: Option<String>,
}

impl AuthorizationResponse {
    pub fn from(authorization_attempt: AuthorizationAttempt) -> Self {
        Self {
            code: authorization_attempt.code,
            state: authorization_attempt.state,
        }
    }

    pub fn redirect_uri(&self, redirect_uri: &str) -> Result<String, ParseError> {
        let mut url = Url::parse(redirect_uri)?;
        url.query_pairs_mut().append_pair("code", &self.code);
        if let Some(s) = self.state.as_ref() {
            url.query_pairs_mut().append_pair("state", s);
        }
        Ok(url.to_string())
    }
}
