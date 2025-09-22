#![allow(dead_code)]

use super::{
    ClientIpsResponse, ClientsStatsResponse, ClientsStatsVecResponse, CpuHistoryResponse,
    DeleteInboundResponse, InboundResponse, InboundsResponse, JsonResponse, NullObjectResponse,
    OnlineClientsResponse, OptStringVecResponse, Result, StringResponse, StringVecResponse,
    UuidRespose,
};
use crate::error::Error;
use crate::models::{ClientRequest, CreateInboundRequest, Inbounds};
use crate::response_ext::ResponseJsonVerboseExt;
use log::debug;
use reqwest::multipart::{Form, Part};
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

    fn gen_url_with_base(&self, base: &[&str], segs: Vec<&str>) -> Result<Url> {
        let base_str = self.url.as_str().trim_end_matches('/');
        let mut url =
            Url::parse(base_str).map_err(|_| Error::InvalidUrl("Invalid base URL".into()))?;
        {
            let mut path_segments = url
                .path_segments_mut()
                .map_err(|_| Error::InvalidUrl("Cannot be a base URL".into()))?;
            path_segments.extend(base.iter().copied());
            path_segments.extend(segs);
        }
        debug!("Generated URL: {}", url);
        Ok(url)
    }

    fn gen_server_url(&self, segs: Vec<&str>) -> Result<Url> {
        self.gen_url_with_base(&["panel", "api", "server"], segs)
    }

    fn gen_inbounds_url(&self, segs: Vec<&str>) -> Result<Url> {
        let base_segs = vec!["panel", "api", "inbounds"];
        self.gen_url_with_base(&base_segs, segs)
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
                return Err(Error::NotFound(response.error_for_status().unwrap_err()));
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
        let res = self.client.get(self.gen_inbounds_url(path)?).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_inbound_by_id(&self, inbound_id: u64) -> Result<InboundResponse> {
        let id = inbound_id.to_string();
        let path = vec!["get", &id];
        let res = self.client.get(self.gen_inbounds_url(path)?).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_client_traffic_by_email(&self, email: String) -> Result<ClientsStatsResponse> {
        let path = vec!["getClientTraffics", &email];
        let res = self.client.get(self.gen_inbounds_url(path)?).send().await?; // todo check is null return user not found
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_client_traffic_by_id(&self, id: String) -> Result<ClientsStatsVecResponse> {
        // todo id to uuid
        let id = id.to_string();
        let path = vec!["getClientTrafficsById", &id];
        let res = self.client.get(self.gen_inbounds_url(path)?).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn send_backup_by_bot(&self) -> Result<()> {
        // todo tests
        let path = vec!["createbackup"];
        let res = self.client.get(self.gen_inbounds_url(path)?).send().await?;
        if res.status() != StatusCode::OK {
            return Err(Error::OtherError("Todo".into()));
        }
        Ok(())
    }

    pub async fn get_client_ips(&self, client_email: String) -> Result<ClientIpsResponse> {
        // todo tests
        let path = vec!["clientIps", &client_email];
        let res = self.client.post(self.gen_inbounds_url(path)?).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn add_inbound(&self, req: &CreateInboundRequest) -> Result<InboundResponse> {
        let url = self.gen_inbounds_url(vec!["add"])?;
        let res = self.client.post(url).json(req).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn add_client_to_inbound(&self, req: &ClientRequest) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec!["addClient"])?;
        let res = self.client.post(url).json(req).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn update_inbound(
        &self,
        inbound_id: u64,
        req: &CreateInboundRequest,
    ) -> Result<InboundResponse> {
        let url = self.gen_inbounds_url(vec!["update", &inbound_id.to_string()])?;
        let res = self.client.post(url).json(req).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn update_client(
        &self,
        uuid: &str,
        req: &ClientRequest,
    ) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec!["updateClient", uuid])?;
        let res = self.client.post(url).json(req).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn clear_client_ips(&self, email: &str) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec!["clearClientIps", email])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn reset_all_inbound_traffics(&self) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec!["resetAllTraffics"])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn reset_all_client_traffics(&self, inbound_id: u64) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec!["resetAllClientTraffics", &inbound_id.to_string()])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn reset_client_traffic(
        &self,
        inbound_id: u64,
        email: &str,
    ) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec![&inbound_id.to_string(), "resetClientTraffic", email])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn delete_client(&self, inbound_id: u64, uuid: &str) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec![&inbound_id.to_string(), "delClient", uuid])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn delete_inbound(&self, inbound_id: u64) -> Result<DeleteInboundResponse> {
        let url = self.gen_inbounds_url(vec!["del", &inbound_id.to_string()])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn delete_depleted_clients(&self, inbound_id: u64) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec!["delDepletedClients", &inbound_id.to_string()])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn online_clients(&self) -> Result<OnlineClientsResponse> {
        let url = self.gen_inbounds_url(vec!["onlines"])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    async fn import_inbound(&self, inbound: &Inbounds) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec!["import"])?;
        let json_str = serde_json::to_string(inbound)
            .map_err(|e| Error::OtherError(format!("serialize inbound: {e}")))?;
        let form = Form::new().text("data", json_str);
        let res = self.client.post(url).multipart(form).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_last_online(&self) -> Result<OptStringVecResponse> {
        let url = self.gen_inbounds_url(vec!["onlines"])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    async fn del_client_by_email(
        &self,
        inbound_id: u64,
        email: &str,
    ) -> Result<NullObjectResponse> {
        let url = self.gen_inbounds_url(vec![&inbound_id.to_string(), "delClientByEmail", email])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn server_status(&self) -> Result<JsonResponse> {
        let url = self.gen_server_url(vec!["status"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn server_get_db(&self) -> Result<Vec<u8>> {
        let url = self.gen_server_url(vec!["getDb"])?;
        let res = self.client.get(url).send().await?;
        Ok(res.bytes().await?.to_vec())
    }

    pub async fn get_xray_version(&self) -> Result<StringVecResponse> {
        let url = self.gen_server_url(vec!["getXrayVersion"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_config_json(&self) -> Result<JsonResponse> {
        let url = self.gen_server_url(vec!["getConfigJson"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn cpu_history(&self, minutes: u32) -> Result<CpuHistoryResponse> {
        let url = self.gen_server_url(vec!["cpuHistory", &minutes.to_string()])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_new_uuid(&self) -> Result<UuidRespose> {
        let url = self.gen_server_url(vec!["getNewUUID"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_new_x25519_cert(&self) -> Result<JsonResponse> {
        let url = self.gen_server_url(vec!["getNewX25519Cert"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_new_mldsa65(&self) -> Result<JsonResponse> {
        let url = self.gen_server_url(vec!["getNewmldsa65"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_new_mlkem768(&self) -> Result<JsonResponse> {
        let url = self.gen_server_url(vec!["getNewmlkem768"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn get_new_vless_enc(&self) -> Result<JsonResponse> {
        let url = self.gen_server_url(vec!["getNewVlessEnc"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    async fn get_new_ech_cert(&self) -> Result<JsonResponse> {
        let url = self.gen_server_url(vec!["getNewEchCert"])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn stop_xray_service(&self) -> Result<NullObjectResponse> {
        let url = self.gen_server_url(vec!["stopXrayService"])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn restart_xray_service(&self) -> Result<NullObjectResponse> {
        let url = self.gen_server_url(vec!["restartXrayService"])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn install_xray_version(&self, version: &str) -> Result<NullObjectResponse> {
        let url = self.gen_server_url(vec!["installXray", version])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn update_geofile(&self) -> Result<NullObjectResponse> {
        let url = self.gen_server_url(vec!["updateGeofile"])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    async fn update_geofile_by_name(&self, file_name: &str) -> Result<NullObjectResponse> {
        let url = self.gen_server_url(vec!["updateGeofile", file_name])?;
        let res = self.client.post(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    async fn logs(&self, count: u32) -> Result<StringResponse> {
        let url = self.gen_server_url(vec!["logs", &count.to_string()])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    async fn xray_logs(&self, count: u32) -> Result<StringResponse> {
        let url = self.gen_server_url(vec!["xraylogs", &count.to_string()])?;
        let res = self.client.get(url).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }

    pub async fn import_db_upload(&self, filename: &str, bytes: Vec<u8>) -> Result<StringResponse> {
        let url = self.gen_server_url(vec!["importDB"])?;
        let form = Form::new().part("db", Part::bytes(bytes).file_name(filename.to_string()));
        let res = self.client.post(url).multipart(form).send().await?;
        res.json_verbose().await.map_err(Into::into)
    }
}
