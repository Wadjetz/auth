use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateApplication {
    pub name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub redirect_uri: String,
}

impl CreateApplication {
    pub fn new(name: String, redirect_uri: String) -> Self {
        Self {
            name,
            description: None,
            website_url: None,
            redirect_uri,
        }
    }
}
