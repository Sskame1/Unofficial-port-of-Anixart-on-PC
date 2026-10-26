use crate::api::{config, error::ApiError};
use reqwest::{
    Url,
    blocking::{Client, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    seed: Url,
}

impl ApiClient {
    pub fn new() -> Result<Self, ApiError> {
        let mut builder = Client::builder()
            .timeout(config::REQUEST_TIMEOUT)
            .user_agent(config::USER_AGENT);

        // reqwest otherwise inherits HTTP_PROXY/HTTPS_PROXY/ALL_PROXY. Some
        // launch environments inject a loopback sink proxy (for example
        // 127.0.0.1:9), which silently makes every API call wait until timeout.
        // Use direct networking by default and allow an explicit proxy opt-in.
        if let Ok(proxy_url) = std::env::var("ANIXART_HTTP_PROXY") {
            let proxy = reqwest::Proxy::all(&proxy_url).map_err(ApiError::Transport)?;
            builder = builder.proxy(proxy);
        } else {
            builder = builder.no_proxy();
        }

        let http = builder.build().map_err(ApiError::Transport)?;
        let seed_value =
            std::env::var("ANIXART_API_URL").unwrap_or_else(|_| config::API_SEED.to_owned());
        let mut seed =
            Url::parse(&seed_value).map_err(|error| ApiError::InvalidUrl(error.to_string()))?;
        if !seed.path().ends_with('/') {
            seed.set_path(&format!("{}/", seed.path()));
        }
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

    pub fn post_json_absolute<B: Serialize, T: DeserializeOwned>(
        &self,
        url: Url,
        body: &B,
    ) -> Result<T, ApiError> {
        let response = self
            .http
            .post(url)
            .json(body)
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

    pub fn post_form<T: DeserializeOwned>(
        &self,
        url: Url,
        fields: &[(&str, &str)],
        header: Option<(&str, &str)>,
    ) -> Result<T, ApiError> {
        self.post_form_with_timeout(url, fields, header, config::REQUEST_TIMEOUT)
    }

    pub fn post_form_with_timeout<T: DeserializeOwned>(
        &self,
        url: Url,
        fields: &[(&str, &str)],
        header: Option<(&str, &str)>,
        timeout: Duration,
    ) -> Result<T, ApiError> {
        let mut request = self.http.post(url).form(fields).timeout(timeout);
        if let Some((name, value)) = header {
            request = request.header(name, value);
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
        if bytes.is_empty() {
            return Err(ApiError::InvalidResponse(
                "API returned an empty response body.".to_owned(),
            ));
        }
        serde_json::from_slice(&bytes).map_err(ApiError::InvalidJson)
    }

    pub fn post_form_absolute<T: DeserializeOwned>(
        &self,
        url: Url,
        fields: &[(&str, &str)],
    ) -> Result<T, ApiError> {
        self.post_form(url, fields, None)
    }
}
