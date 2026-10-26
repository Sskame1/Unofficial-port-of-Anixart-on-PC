use crate::api::{client::ApiClient, error::ApiError};
use reqwest::Url;
use serde::Deserialize;

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
    #[serde(
        default,
        deserialize_with = "crate::features::episodes::string_or_default"
    )]
    name: String,
    #[serde(
        default,
        deserialize_with = "crate::features::episodes::string_or_default"
    )]
    url: String,
    #[serde(default)]
    iframe: bool,
}

fn config_api_urls(client: &ApiClient) -> Result<Vec<String>, ApiError> {
    let mut url = client.url("config/urls")?;
    url.query_pairs_mut()
        .append_pair(
            "version_code",
            &crate::api::config::VERSION_CODE.to_string(),
        )
        .append_pair("is_beta", "false")
        .append_pair("token", "");
    let config: ConfigUrls = client.get_json(url)?;
    let mut urls = config.api_urls;
    if urls.is_empty() {
        urls.push(client.seed().as_str().to_owned());
    }
    Ok(urls)
}

fn resolve_with_base(
    client: &ApiClient,
    base: &str,
    release_id: u64,
    type_id: u64,
    source_id: u64,
    position: u32,
) -> Result<EpisodeLink, ApiError> {
    let base = UrlBase::new(base)?;

    let mut types_url = base.endpoint(&format!("episode/{release_id}"))?;
    types_url.query_pairs_mut().append_pair("token", "");
    let types: TypesResponse = client.get_json(types_url)?;
    if types.response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected the type request (code {}).",
            types.response.code
        )));
    }
    if !types.types.iter().any(|item| item.id == type_id) {
        return Err(ApiError::InvalidResponse(format!(
            "Voice-over/subtitle type {type_id} is not available for release {release_id}."
        )));
    }

    let sources: SourcesResponse =
        client.get_json(base.endpoint(&format!("episode/{release_id}/{type_id}"))?)?;
    if sources.response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected the source request (code {}).",
            sources.response.code
        )));
    }
    if !sources.sources.iter().any(|item| item.id == source_id) {
        return Err(ApiError::InvalidResponse(format!(
            "Source {source_id} is not available for this release and type."
        )));
    }

    let mut episodes_url = base.endpoint(&format!("episode/{release_id}/{type_id}/{source_id}"))?;
    episodes_url
        .query_pairs_mut()
        .append_pair("sort", "0")
        .append_pair("token", "");
    let episodes: EpisodesResponse = client.get_json(episodes_url)?;
    if episodes.response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected the episode request (code {}).",
            episodes.response.code
        )));
    }
    let episode = episodes
        .episodes
        .into_iter()
        .find(|episode| episode.position == position)
        .ok_or_else(|| {
            ApiError::InvalidResponse(format!("Episode position {position} was not found."))
        })?;

    let parsed = reqwest::Url::parse(&episode.url)
        .map_err(|error| ApiError::InvalidUrl(error.to_string()))?;
    if parsed.scheme() != "https" || parsed.host_str() != Some("kodikplayer.com") || !episode.iframe
    {
        return Err(ApiError::InvalidResponse(
            "This MVP currently supports HTTPS Kodik iframe episodes only.".to_owned(),
        ));
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

struct UrlBase(Url);

impl UrlBase {
    fn new(base: &str) -> Result<Self, ApiError> {
        let normalized = format!("{}/", base.trim_end_matches('/'));
        Url::parse(&normalized)
            .map(Self)
            .map_err(|error| ApiError::InvalidUrl(error.to_string()))
    }

    fn endpoint(&self, path: &str) -> Result<Url, ApiError> {
        self.0
            .join(path)
            .map_err(|error| ApiError::InvalidUrl(error.to_string()))
    }
}

pub fn resolve_episode(
    client: &ApiClient,
    release_id: u64,
    type_id: u64,
    source_id: u64,
    position: u32,
) -> Result<EpisodeLink, ApiError> {
    if release_id == 0 || type_id == 0 || source_id == 0 || position == 0 {
        return Err(ApiError::InvalidResponse(
            "All IDs and the episode position must be greater than zero.".to_owned(),
        ));
    }

    let urls = config_api_urls(client)?;
    let mut last_error = None;
    for base in urls {
        match resolve_with_base(client, &base, release_id, type_id, source_id, position) {
            Ok(episode) => return Ok(episode),
            Err(error) => last_error = Some(error),
        }
    }

    Err(last_error.unwrap_or_else(|| {
        ApiError::InvalidResponse("Anixart returned no API addresses.".to_owned())
    }))
}
