mod request;
mod error;
pub mod axum;

use std::env;
use prost::Message;
use reqwest::{header::CONTENT_TYPE, Client};

pub use request::CreateProjectRequest;
pub use request::CreateUserRequest;
pub use error::Error;

pub struct SearchClient {
    http_client: Client,
    base_url: String,
}

impl SearchClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/user", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn create_project(&self, request: CreateProjectRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/project", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

impl Default for SearchClient {
    fn default() -> Self {
        let base_url = env::var("SEARCH_BASE_URL")
            .unwrap_or("http://search".to_string())
            .trim_end_matches('/')
            .to_string();

        Self { 
            http_client: Default::default(),
            base_url
        }
    }
}