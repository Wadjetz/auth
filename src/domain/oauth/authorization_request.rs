use itertools::join;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationRequest {
    pub client_id: String,
    pub response_type: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
}

impl AuthorizationRequest {
    pub fn to_querystring(&self) -> String {
        let params = vec![
            ("client_id", Some(self.client_id.as_str())),
            ("response_type", Some(self.response_type.as_str())),
            ("redirect_uri", Some(self.redirect_uri.as_str())),
            ("scope", self.scope.as_deref()),
            ("state", self.state.as_deref()),
        ];
        let params = params
            .iter()
            .filter_map(|(key, value)| value.map(|v| format!("{}={}", *key, &v)));
        join(params, "&")
    }
}
