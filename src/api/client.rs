use crate::api::{config, error::ApiError};
use reqwest::{
    Url,
    blocking::{Client, Response},
};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    seed: Url,
}

impl ApiClient {
    pub fn new() -> Result<Self, ApiError> {
        let http = Client::builder()
            .timeout(config::REQUEST_TIMEOUT)
            .user_agent(config::USER_AGENT)
            .build()
            .map_err(ApiError::Transport)?;
        let seed = Url::parse(config::API_SEED)
            .map_err(|error| ApiError::InvalidUrl(error.to_string()))?;
        Ok(Self { http, seed })
    }

    pub fn seed(&self) -> &Url {
        &self.seed
    }

    pub fn url(&self, path: &str) -> Result<Url, ApiError> {
        self.seed
            .join(path)
            .map_err(|error| ApiError::InvalidUrl(error.to_string()))
    }

    pub fn get(&self, url: Url) -> Result<Response, ApiError> {
        self.http
            .get(url)
            .send()
            .map_err(ApiError::Transport)?
            .error_for_status()
            .map_err(|error| {
                if let Some(status) = error.status() {
                    ApiError::HttpStatus(status)
                } else {
                    ApiError::Transport(error)
                }
            })
    }

    pub fn get_json<T: DeserializeOwned>(&self, url: Url) -> Result<T, ApiError> {
        let response = self.get(url)?;
        let bytes = response.bytes().map_err(ApiError::Transport)?;
        serde_json::from_slice(&bytes).map_err(ApiError::InvalidJson)
    }

    pub fn post_empty_json<T: DeserializeOwned>(&self, url: Url) -> Result<T, ApiError> {
        let response = self
            .http
            .post(url)
            .send()
            .map_err(ApiError::Transport)?
            .error_for_status()
            .map_err(|error| {
                if let Some(status) = error.status() {
                    ApiError::HttpStatus(status)
                } else {
                    ApiError::Transport(error)
                }
            })?;
        let bytes = response.bytes().map_err(ApiError::Transport)?;
        serde_json::from_slice(&bytes).map_err(ApiError::InvalidJson)
    }

    pub fn post_json_with_header<B: Serialize, T: DeserializeOwned>(
        &self,
        url: Url,
        body: &B,
        header: &str,
        value: &str,
    ) -> Result<T, ApiError> {
        let mut request = self.http.post(url).json(body);
        if !header.is_empty() {
            request = request.header(header, value);
        }
        let response = request
            .send()
            .map_err(ApiError::Transport)?
            .error_for_status()
            .map_err(|error| {
                if let Some(status) = error.status() {
                    ApiError::HttpStatus(status)
                } else {
                    ApiError::Transport(error)
                }
            })?;
        let bytes = response.bytes().map_err(ApiError::Transport)?;
        serde_json::from_slice(&bytes).map_err(ApiError::InvalidJson)
    }
}
