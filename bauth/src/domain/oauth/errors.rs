use itertools::join;
use std::fmt;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum OauthErrorKind {
    #[error("invalid_request")]
    InvalidRequest,
    #[error("unauthorized_client")]
    UnauthorizedClient,
    #[error("access_denied")]
    AccessDenied,
    #[error("unsupported_response_type")]
    UnsupportedResponseType,
    #[error("invalid_scope")]
    InvalidScope,
    #[error("server_error")]
    ServerError,
    #[error("temporarily_unavailable")]
    TemporarilyUnavailable,
}

#[derive(Error, Debug)]
pub struct OauthError {
    kind: OauthErrorKind,
    redirect_uri: String,
    description: Option<String>,
    uri: Option<String>,
}

impl OauthError {
    pub fn new(
        kind: OauthErrorKind,
        redirect_uri: String,
        description: Option<String>,
        uri: Option<String>,
    ) -> Self {
        Self {
            kind,
            redirect_uri,
            description,
            uri,
        }
    }

    pub fn to_querystring(&self) -> String {
        let kind = self.kind.to_string();
        let params = vec![
            ("error", Some(kind.as_str())),
            ("error_description", self.description.as_deref()),
            ("error_uri", self.uri.as_deref()),
        ];
        let params = params
            .iter()
            .filter_map(|(key, value)| value.map(|v| format!("{}={}", *key, &v)));
        join(params, "&")
    }

    pub fn to_redirect_url(&self) -> String {
        format!("{}?{}", self.redirect_uri, self.to_querystring())
    }
}

impl fmt::Display for OauthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
