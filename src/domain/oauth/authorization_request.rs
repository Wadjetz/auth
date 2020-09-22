use itertools::join;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use url::Url;

#[derive(Debug, EnumString, Display, Deserialize, Serialize)]
pub enum ResponseType {
    #[strum(serialize = "code")]
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "token")]
    #[strum(serialize = "token")]
    Token,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationRequest {
    pub client_id: String,
    pub response_type: ResponseType,
    pub redirect_uri: Url,
    pub scope: Option<String>,
    pub state: Option<String>,
}

impl AuthorizationRequest {
    pub fn to_querystring(&self) -> String {
        let response_type = self.response_type.to_string();
        let params = vec![
            ("client_id", Some(self.client_id.as_str())),
            ("response_type", Some(&response_type)),
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
