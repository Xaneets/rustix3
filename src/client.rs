use super::Result;
use crate::error::Error;
use crate::models::inbounds::InboundsResponse;
use crate::models::LoginResponse;
use log::debug;
use reqwest::{Client as RClient, IntoUrl, StatusCode, Url};
use serde::Serialize;

#[derive(Debug)]
pub struct Client {
    username: String,
    password: String,
    url: Url,
    client: RClient,
}

impl Client {
    pub async fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        url: impl IntoUrl,
    ) -> Result<Self> {
        let client = Self {
            username: username.into(),
            password: password.into(),
            url: url.into_url()?,
            client: RClient::builder().cookie_store(true).build()?,
        };
        debug!("{:?}", client);
        let res = client.login().await?;
        if res.is_err() {
            return Err(Error::InvalidCred);
        }
        Ok(client)
    }

    async fn login(&self) -> Result<LoginResponse> {
        #[derive(Serialize)]
        struct LoginRequest {
            username: String,
            password: String,
        }
        let body = LoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
        };

        debug!("Sending login request!");
        let response = self
            .client
            .post(self.url.clone().join("login").unwrap())
            .json(&body)
            .send()
            .await?;
        match response.status() {
            StatusCode::NOT_FOUND => {
                return Err(Error::NotFound(response.error_for_status().unwrap_err()))
            }
            StatusCode::OK => {}
            e => {
                log::warn!("Unimplemented handle err{:?}", e)
            }
        }
        Ok(response.json().await?)
    }

    pub async fn get_inbounds_list(&self) -> Result<InboundsResponse> {
        let mut url = self.url.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| Error::InvalidUrl("Cannot be a base URL".into()))?;
            segments.clear();
            segments.extend(&["panel", "api", "inbounds", "list"]);
        }

        let res = self.client.get(url).send().await?;
        Ok(res.json().await?)
    }

    pub async fn get_inbound_by_id(&self, inbound_id: u64) -> Result<InboundsResponse> {
        let mut url = self.url.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| Error::InvalidUrl("Cannot be a base URL".into()))?;
            segments.clear();
            segments.extend(&["panel", "api", "inbounds", "get", &inbound_id.to_string()]);
        }

        let res = self.client.get(url).send().await?;
        Ok(res.json().await?)
    }
}
