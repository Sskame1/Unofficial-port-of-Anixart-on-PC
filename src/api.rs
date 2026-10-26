use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

const API_SEED: &str = "https://api-s.anixsekai.com/";
const VERSION_CODE: u32 = 26_101_718;
const USER_AGENT: &str = "AnixartWindowsClient/0.1 (Windows; x64)";

#[derive(Debug)]
pub struct EpisodeLink {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct ConfigUrls {
    #[serde(default)]
    api_urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CodeResponse {
    code: i32,
}

#[derive(Debug, Deserialize)]
struct TypesResponse {
    #[serde(flatten)]
    response: CodeResponse,
    #[serde(default)]
    types: Vec<TypeItem>,
}

#[derive(Debug, Deserialize)]
struct TypeItem {
    id: u64,
}

#[derive(Debug, Deserialize)]
struct SourcesResponse {
    #[serde(flatten)]
    response: CodeResponse,
    #[serde(default)]
    sources: Vec<SourceItem>,
}

#[derive(Debug, Deserialize)]
struct SourceItem {
    id: u64,
}

#[derive(Debug, Deserialize)]
struct EpisodesResponse {
    #[serde(flatten)]
    response: CodeResponse,
    #[serde(default)]
    episodes: Vec<EpisodeItem>,
}

#[derive(Debug, Deserialize)]
struct EpisodeItem {
    position: u32,
    #[serde(default)]
    name: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    iframe: bool,
}

fn get_json<T: for<'de> Deserialize<'de>>(client: &Client, url: &str) -> Result<T> {
    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .with_context(|| format!("request failed: {url}"))?
        .error_for_status()
        .with_context(|| format!("API returned an HTTP error: {url}"))?;
    response
        .json::<T>()
        .with_context(|| format!("invalid JSON returned by API: {url}"))
}

fn config_api_urls(client: &Client) -> Result<Vec<String>> {
    let config_url = format!(
        "{API_SEED}config/urls?version_code={VERSION_CODE}&is_beta=false&token="
    );
    let config: ConfigUrls = get_json(client, &config_url)?;
    let mut urls = config.api_urls;
    if urls.is_empty() {
        urls.push(API_SEED.to_owned());
    }
    Ok(urls)
}

fn resolve_with_base(
    client: &Client,
    base: &str,
    release_id: u64,
    type_id: u64,
    source_id: u64,
    position: u32,
) -> Result<EpisodeLink> {
    let base = format!("{}/", base.trim_end_matches('/'));

    let types_url = format!("{base}episode/{release_id}?token=");
    let types: TypesResponse = get_json(client, &types_url)?;
    if types.response.code != 0 {
        bail!("Anixart rejected the type request (code {}).", types.response.code);
    }
    if !types.types.iter().any(|item| item.id == type_id) {
        bail!("Voice-over/subtitle type {type_id} is not available for release {release_id}.");
    }

    let sources_url = format!("{base}episode/{release_id}/{type_id}?token=");
    let sources: SourcesResponse = get_json(client, &sources_url)?;
    if sources.response.code != 0 {
        bail!("Anixart rejected the source request (code {}).", sources.response.code);
    }
    if !sources.sources.iter().any(|item| item.id == source_id) {
        bail!("Source {source_id} is not available for this release and type.");
    }

    let episodes_url = format!("{base}episode/{release_id}/{type_id}/{source_id}?sort=0&token=");
    let episodes: EpisodesResponse = get_json(client, &episodes_url)?;
    if episodes.response.code != 0 {
        bail!("Anixart rejected the episode request (code {}).", episodes.response.code);
    }
    let episode = episodes
        .episodes
        .into_iter()
        .find(|episode| episode.position == position)
        .with_context(|| format!("Episode position {position} was not found."))?;

    let parsed = reqwest::Url::parse(&episode.url).context("Anixart returned an invalid episode URL.")?;
    if parsed.scheme() != "https" || parsed.host_str() != Some("kodikplayer.com") || !episode.iframe {
        bail!("This MVP currently supports HTTPS Kodik iframe episodes only.");
    }

    Ok(EpisodeLink {
        name: if episode.name.is_empty() {
            format!("Episode {position}")
        } else {
            episode.name
        },
        url: episode.url,
    })
}

pub fn resolve_episode(
    release_id: u64,
    type_id: u64,
    source_id: u64,
    position: u32,
) -> Result<EpisodeLink> {
    if release_id == 0 || type_id == 0 || source_id == 0 || position == 0 {
        bail!("All IDs and the episode position must be greater than zero.");
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .context("could not initialize HTTP client")?;
    let urls = config_api_urls(&client)?;
    let mut last_error = None;

    for base in urls {
        match resolve_with_base(&client, &base, release_id, type_id, source_id, position) {
            Ok(episode) => return Ok(episode),
            Err(error) => last_error = Some(error),
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Anixart returned no API addresses.")))
}
