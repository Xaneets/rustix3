use super::{ClientIpsResponse, ClientsStatsResponse, ClientsStatsVecResponse, InboundResponse, InboundsResponse, NullObjectResponse, Result};
use crate::error::Error;
use log::{debug, error};
use reqwest::{Client as RClient, IntoUrl, StatusCode, Url};
use serde::Serialize;

type LoginResponse = NullObjectResponse;

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

    fn gen_url(&self, segs: Vec<&str>) -> Result<Url> {
        // todo paths to hashmap or enum
        let mut base_segs = vec!["panel", "api", "inbounds"];
        base_segs.extend(segs);
        let base = self.url.as_str().trim_end_matches('/');
        let mut url = Url::parse(base).map_err(|_| Error::InvalidUrl("Invalid base URL".into()))?;

        {
            let mut path_segments = url
                .path_segments_mut()
                .map_err(|_| Error::InvalidUrl("Cannot be a base URL".into()))?;
            path_segments.extend(base_segs);
        }
        debug!("Generated URL: {}", url);
        Ok(url)
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
        let path = vec!["list"];
        let res = self.client.get(self.gen_url(path)?).send().await?;
        let res = res.json().await.map_err(|e| {
            error!("{e}");
            e
        })?;
        Ok(res)
    }

    pub async fn get_inbound_by_id(&self, inbound_id: u64) -> Result<InboundResponse> {
        let id = inbound_id.to_string();
        let path = vec!["get", &id];
        let res = self.client.get(self.gen_url(path)?).send().await?;
        Ok(res.json().await?)
    }

    pub async fn get_client_traffic_by_email(&self, email: String) -> Result<ClientsStatsResponse> {
        let path = vec!["getClientTraffics", &email];
        let res = self.client.get(self.gen_url(path)?).send().await?; // todo check is null return user not found
        Ok(res.json().await?)
    }

    pub async fn get_client_traffic_by_id(&self, id: String) -> Result<ClientsStatsVecResponse> { // todo id to uuid
        let id = id.to_string();
        let path = vec!["getClientTrafficsById", &id];
        let res = self.client.get(self.gen_url(path)?).send().await?;
        Ok(res.json().await?)
    }

    pub async fn send_backup_by_bot(&self) -> Result<()> {
        let path = vec!["createbackup"];
        let res = self.client.get(self.gen_url(path)?).send().await?;
        if res.status() != StatusCode::OK {
            return Err(Error::OtherError("Todo".into()));
        }
        Ok(())
    }

    pub async fn get_client_ips(&self, client_email: String) -> Result<ClientIpsResponse> {
        let path = vec!["clientIps", &client_email];
        let res = self.client.post(self.gen_url(path)?).send().await?;
        Ok(res.json().await?)
    }
}
