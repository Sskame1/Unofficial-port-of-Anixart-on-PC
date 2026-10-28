use chrono::Local;
use reqwest::{blocking::Client, header, Url};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use wry::http::{Request, Response, StatusCode, header::CONTENT_TYPE};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    date: String,
    category: String,
    file: String,
    mime: String,
    etag: Option<String>,
    last_modified: Option<String>,
    checked_at: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct CacheIndex {
    entries: HashMap<String, CacheEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheConfig {
    enabled: bool,
    #[serde(default = "default_retention_days")]
    retention_days: u32,
}

fn default_retention_days() -> u32 { 90 }

pub struct ImageCache {
    root: PathBuf,
    enabled: Mutex<bool>,
    index: Arc<Mutex<CacheIndex>>,
    http: Client,
}

impl ImageCache {
    pub fn new() -> Result<Arc<Self>, String> {
        let local = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let root = local.join("AnixartWindowsClient").join("Memory");
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        let config_path = local.join("AnixartWindowsClient").join("image-cache.json");
        let config = fs::read(&config_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<CacheConfig>(&bytes).ok())
            .unwrap_or(CacheConfig { enabled: true, retention_days: default_retention_days() });
        cleanup_old_days(&root, config.retention_days);
        let index_path = root.join("index.json");
        let index = fs::read(&index_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<CacheIndex>(&bytes).ok())
            .unwrap_or_default();
        let http = Client::builder()
            .timeout(Duration::from_secs(25))
            .user_agent(crate::api::config::USER_AGENT)
            .no_proxy()
            .build()
            .map_err(|error| error.to_string())?;
        Ok(Arc::new(Self { root, enabled: Mutex::new(config.enabled), index: Arc::new(Mutex::new(index)), http }))
    }

    pub fn enabled(&self) -> bool {
        self.enabled.lock().map(|value| *value).unwrap_or(false)
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<(), String> {
        if let Ok(mut value) = self.enabled.lock() { *value = enabled; }
        let app_dir = self.root.parent().ok_or("Invalid cache directory")?;
        fs::create_dir_all(app_dir).map_err(|error| error.to_string())?;
        let path = app_dir.join("image-cache.json");
        let bytes = serde_json::to_vec_pretty(&CacheConfig { enabled, retention_days: default_retention_days() })
            .map_err(|error| error.to_string())?;
        fs::write(path, bytes).map_err(|error| error.to_string())
    }

    pub fn load_data_url(&self, source: &str, category: &str) -> Result<String, String> {
        if !self.enabled() {
            return Err("Image cache is disabled".to_owned());
        }
        let (mime, bytes) = self.load_image(source, &safe_category(category))?;
        Ok(format!(
            "data:{mime};base64,{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
        ))
    }

    pub fn respond(&self, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
        let raw_uri = request.uri().to_string();
        let parsed = Url::parse(&raw_uri).or_else(|_| Url::parse(&format!("https://cache.local{raw_uri}")))
            .map_err(|error| error.to_string())
            .and_then(|url| {
                let mut source = None;
                let mut category = None;
                for (name, value) in url.query_pairs() {
                    match name.as_ref() {
                        "url" => source = Some(value.into_owned()),
                        "category" => category = Some(value.into_owned()),
                        _ => {}
                    }
                }
                let source = source.ok_or_else(|| "Missing image URL".to_owned())?;
                let category = safe_category(category.as_deref().unwrap_or("posters"));
                self.load_image(&source, &category)
            });
        match parsed {
            Ok((mime, bytes)) => Response::builder().status(StatusCode::OK)
                .header(CONTENT_TYPE, mime).header(header::CACHE_CONTROL, "public, max-age=86400")
                .body(Cow::Owned(bytes)).unwrap_or_else(|_| error_response(StatusCode::INTERNAL_SERVER_ERROR, "Could not build image response")),
            Err(error) => error_response(StatusCode::BAD_GATEWAY, &error),
        }
    }

    fn load_image(&self, source: &str, category: &str) -> Result<(String, Vec<u8>), String> {
        let url = Url::parse(source).map_err(|error| error.to_string())?;
        if url.scheme() != "https" || !matches!(url.host_str(), Some("s.anixmirai.com" | "anixart.ru" | "anixart.app" | "anixart.tv" | "anixsekai.com" | "kodikplayer.com" | "kodik.info" | "kodik.biz" | "kodik.cc" | "kodik.club" | "kodik.ws" | "kodikid.com" | "kodikapi.com" | "kodikres.com" | "anixart-images.s3.amazonaws.com")) {
            // Image URLs from feed/user content are accepted on HTTPS hosts.
            if url.scheme() != "https" { return Err("Only HTTPS images are allowed".to_owned()); }
        }
        let hash = format!("{:x}", Sha256::digest(source.as_bytes()));
        let now = chrono::Utc::now().timestamp();
        let old = if self.enabled() {
            self.index.lock().ok().and_then(|index| index.entries.get(&hash).cloned())
        } else { None };
        let old_path = old.as_ref().map(|entry| self.root.join(&entry.file));
        if let (Some(entry), Some(path)) = (old.as_ref(), old_path.as_ref()) {
            if path.is_file() {
                if now.saturating_sub(entry.checked_at) >= 86_400 {
                    let root = self.root.clone();
                    let index = self.index.clone();
                    let http = self.http.clone();
                    let source = source.to_owned();
                    let hash = hash.clone();
                    let category = category.to_owned();
                    let old = entry.clone();
                    std::thread::spawn(move || {
                        let _ = refresh_cached_image(&root, &index, &http, &source, &category, &hash, old);
                    });
                }
                return Ok((entry.mime.clone(), fs::read(path).map_err(|error| error.to_string())?));
            }
        }

        let mut request = self.http.get(url);
        if let Some(entry) = &old {
            if let Some(etag) = &entry.etag { request = request.header(header::IF_NONE_MATCH, etag); }
            if let Some(modified) = &entry.last_modified { request = request.header(header::IF_MODIFIED_SINCE, modified); }
        }
        let response = request.send().map_err(|error| error.to_string())?;
        if response.status() == StatusCode::NOT_MODIFIED {
            if let (Some(mut entry), Some(path)) = (old, old_path) {
                entry.checked_at = now;
                self.update_index(hash, entry.clone())?;
                return Ok((entry.mime, fs::read(path).map_err(|error| error.to_string())?));
            }
        }
        let response = response.error_for_status().map_err(|error| error.to_string())?;
        let mime = response.headers().get(CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap_or("application/octet-stream").split(';').next().unwrap_or("application/octet-stream").to_owned();
        if !mime.starts_with("image/") { return Err(format!("Remote URL did not return an image ({mime})")); }
        let etag = response.headers().get(header::ETAG).and_then(|value| value.to_str().ok()).map(str::to_owned);
        let last_modified = response.headers().get(header::LAST_MODIFIED).and_then(|value| value.to_str().ok()).map(str::to_owned);
        let bytes = response.bytes().map_err(|error| error.to_string())?.to_vec();
        let date = Local::now().format("%Y-%m-%d").to_string();
        let extension = match mime.as_str() { "image/png" => "png", "image/webp" => "webp", "image/gif" => "gif", "image/avif" => "avif", "image/svg+xml" => "svg", _ => "jpg" };
        let relative = PathBuf::from(&date).join(category).join(format!("{hash}.{extension}"));
        let path = self.root.join(&relative);
        if self.enabled() {
            if let Some(parent) = path.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
            fs::write(&path, &bytes).map_err(|error| error.to_string())?;
            self.update_index(hash, CacheEntry { date, category: category.to_owned(), file: relative.to_string_lossy().replace('\\', "/"), mime: mime.clone(), etag, last_modified, checked_at: now })?;
        }
        Ok((mime, bytes))
    }

    fn update_index(&self, key: String, entry: CacheEntry) -> Result<(), String> {
        let mut index = self.index.lock().map_err(|_| "Image cache index is locked".to_owned())?;
        index.entries.insert(key, entry);
        let bytes = serde_json::to_vec_pretty(&*index).map_err(|error| error.to_string())?;
        fs::write(self.root.join("index.json"), bytes).map_err(|error| error.to_string())
    }
}

fn refresh_cached_image(
    root: &PathBuf,
    index: &Arc<Mutex<CacheIndex>>,
    http: &Client,
    source: &str,
    category: &str,
    hash: &str,
    mut old: CacheEntry,
) -> Result<(), String> {
    let url = Url::parse(source).map_err(|error| error.to_string())?;
    let mut request = http.get(url);
    if let Some(etag) = &old.etag { request = request.header(header::IF_NONE_MATCH, etag); }
    if let Some(modified) = &old.last_modified { request = request.header(header::IF_MODIFIED_SINCE, modified); }
    let response = request.send().map_err(|error| error.to_string())?;
    let now = chrono::Utc::now().timestamp();
    if response.status() == StatusCode::NOT_MODIFIED {
        old.checked_at = now;
        return write_index(root, index, hash, old);
    }
    let response = response.error_for_status().map_err(|error| error.to_string())?;
    let mime = response.headers().get(CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap_or("application/octet-stream").split(';').next().unwrap_or("application/octet-stream").to_owned();
    if !mime.starts_with("image/") { return Err(format!("Remote URL did not return an image ({mime})")); }
    let etag = response.headers().get(header::ETAG).and_then(|value| value.to_str().ok()).map(str::to_owned);
    let last_modified = response.headers().get(header::LAST_MODIFIED).and_then(|value| value.to_str().ok()).map(str::to_owned);
    let bytes = response.bytes().map_err(|error| error.to_string())?.to_vec();
    let date = Local::now().format("%Y-%m-%d").to_string();
    let extension = match mime.as_str() { "image/png" => "png", "image/webp" => "webp", "image/gif" => "gif", "image/avif" => "avif", "image/svg+xml" => "svg", _ => "jpg" };
    let relative = PathBuf::from(date).join(category).join(format!("{hash}.{extension}"));
    let path = root.join(&relative);
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
    fs::write(path, bytes).map_err(|error| error.to_string())?;
    write_index(root, index, hash, CacheEntry {
        date: Local::now().format("%Y-%m-%d").to_string(), category: category.to_owned(),
        file: relative.to_string_lossy().replace('\\', "/"), mime, etag, last_modified, checked_at: now,
    })
}

fn write_index(root: &PathBuf, index: &Arc<Mutex<CacheIndex>>, key: &str, entry: CacheEntry) -> Result<(), String> {
    let mut index = index.lock().map_err(|_| "Image cache index is locked".to_owned())?;
    index.entries.insert(key.to_owned(), entry);
    let bytes = serde_json::to_vec_pretty(&*index).map_err(|error| error.to_string())?;
    fs::write(root.join("index.json"), bytes).map_err(|error| error.to_string())
}

fn safe_category(category: &str) -> String {
    match category {
        "screenshots" | "discover" | "feed" | "avatars" | "banners" | "collections" | "posters" => category.to_owned(),
        _ => "posters".to_owned(),
    }
}

fn cleanup_old_days(root: &PathBuf, retention_days: u32) {
    let Ok(entries) = fs::read_dir(root) else { return; };
    let today = Local::now().date_naive();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else { continue; };
        let Ok(date) = chrono::NaiveDate::parse_from_str(name, "%Y-%m-%d") else { continue; };
        if today.signed_duration_since(date).num_days() > i64::from(retention_days) {
            let _ = fs::remove_dir_all(path);
        }
    }
}

fn error_response(status: StatusCode, message: &str) -> Response<Cow<'static, [u8]>> {
    Response::builder().status(status).header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Cow::Owned(message.as_bytes().to_vec())).unwrap_or_else(|_| Response::new(Cow::Owned(Vec::new())))
}

