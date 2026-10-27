use crate::api::{config, error::ApiError};
use reqwest::{
    Url,
    blocking::{Client, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    seed: Url,
    active_api_host: Arc<Mutex<String>>,
    api_hosts: Arc<Mutex<Vec<String>>>,
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
        let active_api_host = seed.host_str().unwrap_or_default().to_owned();
        let api_hosts = vec![active_api_host.clone()];
        Ok(Self {
            http,
            seed,
            active_api_host: Arc::new(Mutex::new(active_api_host)),
            api_hosts: Arc::new(Mutex::new(api_hosts)),
        })
    }

    pub fn seed(&self) -> &Url {
        &self.seed
    }

    pub fn url(&self, path: &str) -> Result<Url, ApiError> {
        let mut base = self.seed.clone();
        if let Ok(host) = self.active_api_host.lock() {
            let _ = base.set_host(Some(&host));
        }
        base.join(path)
            .map_err(|error| ApiError::InvalidUrl(error.to_string()))
    }

    pub fn get(&self, url: Url) -> Result<Response, ApiError> {
        let mut candidates = self.api_candidates(&url);
        let mut last_error = None;
        for round in 0..2 {
            for candidate in candidates {
                match self.http.get(candidate.clone()).send() {
                    Ok(response) if should_retry_status(response.status()) => {
                        last_error = Some(ApiError::HttpStatus(response.status()));
                    }
                    Ok(response) => {
                        let response = response.error_for_status().map_err(|error| {
                            error
                                .status()
                                .map(ApiError::HttpStatus)
                                .unwrap_or_else(|| ApiError::Transport(error))
                        })?;
                        self.remember_api_host(&candidate);
                        return Ok(response);
                    }
                    Err(error) => last_error = Some(ApiError::Transport(error)),
                }
            }
            if round == 0 && self.refresh_api_hosts() {
                candidates = self.api_candidates(&url);
            } else {
                break;
            }
        }
        Err(last_error.unwrap_or_else(|| {
            ApiError::InvalidResponse("No Anixart API endpoint is configured.".into())
        }))
    }

    pub fn get_json<T: DeserializeOwned>(&self, url: Url) -> Result<T, ApiError> {
        let response = self.get(url)?;
        let bytes = response.bytes().map_err(ApiError::Transport)?;
        serde_json::from_slice(&bytes).map_err(ApiError::InvalidJson)
    }

    pub fn get_json_with_header<T: DeserializeOwned>(
        &self,
        url: Url,
        header: &str,
        value: &str,
    ) -> Result<T, ApiError> {
        let response = self.send_with_fallback(url, |client, candidate| {
            client.get(candidate).header(header, value).send()
        })?;
        let bytes = response.bytes().map_err(ApiError::Transport)?;
        serde_json::from_slice(&bytes).map_err(ApiError::InvalidJson)
    }

    pub fn post_empty_json<T: DeserializeOwned>(&self, url: Url) -> Result<T, ApiError> {
        let response =
            self.send_with_fallback(url, |client, candidate| client.post(candidate).send())?;
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
        let mut candidates = self.api_candidates(&url);
        let mut last_error = None;
        for round in 0..2 {
            for candidate in candidates {
                let mut request = self.http.post(candidate.clone()).json(body);
                if !header.is_empty() {
                    request = request.header(header, value);
                }
                match request.send() {
                    Ok(response) if should_retry_status(response.status()) => {
                        last_error = Some(ApiError::HttpStatus(response.status()))
                    }
                    Ok(response) => {
                        let response = response.error_for_status().map_err(|error| {
                            error
                                .status()
                                .map(ApiError::HttpStatus)
                                .unwrap_or_else(|| ApiError::Transport(error))
                        })?;
                        self.remember_api_host(&candidate);
                        let bytes = response.bytes().map_err(ApiError::Transport)?;
                        return serde_json::from_slice(&bytes).map_err(ApiError::InvalidJson);
                    }
                    Err(error) => last_error = Some(ApiError::Transport(error)),
                }
            }
            if round == 0 && self.refresh_api_hosts() {
                candidates = self.api_candidates(&url);
            } else {
                break;
            }
        }
        Err(last_error.unwrap_or_else(|| {
            ApiError::InvalidResponse("No Anixart API endpoint is configured.".into())
        }))
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
        let response = self.send_with_fallback(url, |client, candidate| {
            let mut request = client.post(candidate).form(fields).timeout(timeout);
            if let Some((name, value)) = header {
                request = request.header(name, value);
            }
            request.send()
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

    fn api_candidates(&self, url: &Url) -> Vec<Url> {
        let host = match url.host_str() {
            Some(host) if self.is_api_host(host) => host,
            _ => return vec![url.clone()],
        };
        let active = self
            .active_api_host
            .lock()
            .map(|host| host.clone())
            .unwrap_or_else(|_| host.to_owned());
        let mut hosts = vec![active, host.to_owned()];
        if let Ok(discovered) = self.api_hosts.lock() {
            hosts.extend(discovered.iter().cloned());
        }
        hosts.dedup();
        hosts
            .into_iter()
            .filter_map(|host| {
                let mut candidate = url.clone();
                candidate.set_host(Some(&host)).ok()?;
                Some(candidate)
            })
            .collect()
    }

    fn send_with_fallback<F>(&self, url: Url, mut send: F) -> Result<Response, ApiError>
    where
        F: FnMut(&Client, Url) -> Result<Response, reqwest::Error>,
    {
        let mut candidates = self.api_candidates(&url);
        let mut last_error = None;
        for round in 0..2 {
            for candidate in candidates {
                match send(&self.http, candidate.clone()) {
                    Ok(response) if should_retry_status(response.status()) => {
                        last_error = Some(ApiError::HttpStatus(response.status()));
                    }
                    Ok(response) => {
                        let response = response.error_for_status().map_err(|error| {
                            error
                                .status()
                                .map(ApiError::HttpStatus)
                                .unwrap_or_else(|| ApiError::Transport(error))
                        })?;
                        self.remember_api_host(&candidate);
                        return Ok(response);
                    }
                    Err(error) => last_error = Some(ApiError::Transport(error)),
                }
            }
            if round == 0 && self.refresh_api_hosts() {
                candidates = self.api_candidates(&url);
            } else {
                break;
            }
        }
        Err(last_error.unwrap_or_else(|| {
            ApiError::InvalidResponse("No Anixart API endpoint is configured.".into())
        }))
    }

    fn remember_api_host(&self, url: &Url) {
        if let (Some(host), Ok(mut active)) = (url.host_str(), self.active_api_host.lock()) {
            *active = host.to_owned();
        }
    }

    fn is_api_host(&self, host: &str) -> bool {
        host == self.seed.host_str().unwrap_or_default()
            || self
                .api_hosts
                .lock()
                .map(|hosts| hosts.iter().any(|item| item == host))
                .unwrap_or(false)
    }

    // The Android client obtains its live API mirror list from config/urls.
    // Ask that endpoint only after a primary host failure, then retry once.
    fn refresh_api_hosts(&self) -> bool {
        let Ok(url) = self
            .seed
            .join("config/urls?version_code=26101718&is_beta=false&token=")
        else {
            return false;
        };
        let Ok(response) = self.http.get(url).timeout(Duration::from_secs(5)).send() else {
            return false;
        };
        let Ok(payload) = response.error_for_status().and_then(|r| r.json::<Value>()) else {
            return false;
        };
        let Some(urls) = payload.get("api_urls").and_then(Value::as_array) else {
            return false;
        };
        let mut hosts = vec![self.seed.host_str().unwrap_or_default().to_owned()];
        for value in urls.iter().filter_map(Value::as_str) {
            let Ok(parsed) = Url::parse(value) else {
                continue;
            };
            if parsed.scheme() != "https" {
                continue;
            }
            let Some(host) = parsed.host_str() else {
                continue;
            };
            if !hosts.iter().any(|existing| existing == host) {
                hosts.push(host.to_owned());
            }
        }
        if let Ok(mut stored) = self.api_hosts.lock() {
            *stored = hosts.clone();
        }
        hosts.len() > 1
    }
}

fn should_retry_status(status: reqwest::StatusCode) -> bool {
    status.is_server_error() || matches!(status.as_u16(), 403 | 429)
}
