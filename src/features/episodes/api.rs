use crate::{
    api::{client::ApiClient, error::ApiError},
    features::episodes::{EpisodeItem, EpisodeSource, EpisodeType},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct TypesResponse {
    code: i32,
    #[serde(default)]
    types: Vec<EpisodeType>,
}

#[derive(Deserialize)]
struct SourcesResponse {
    code: i32,
    #[serde(default)]
    sources: Vec<EpisodeSource>,
}

#[derive(Deserialize)]
struct EpisodesResponse {
    code: i32,
    #[serde(default)]
    episodes: Vec<EpisodeItem>,
}

pub fn types(client: &ApiClient, release_id: u64) -> Result<Vec<EpisodeType>, ApiError> {
    let mut url = client.url(&format!("episode/{release_id}"))?;
    url.query_pairs_mut().append_pair("token", "");
    let response: TypesResponse = client.get_json(url)?;
    ensure_success(response.code, "types")?;
    Ok(response.types)
}

pub fn sources(
    client: &ApiClient,
    release_id: u64,
    type_id: u64,
) -> Result<Vec<EpisodeSource>, ApiError> {
    let mut url = client.url(&format!("episode/{release_id}/{type_id}"))?;
    url.query_pairs_mut().append_pair("token", "");
    let response: SourcesResponse = client.get_json(url)?;
    ensure_success(response.code, "sources")?;
    Ok(response.sources)
}

pub fn episodes(
    client: &ApiClient,
    release_id: u64,
    type_id: u64,
    source_id: u64,
) -> Result<Vec<EpisodeItem>, ApiError> {
    let mut url = client.url(&format!("episode/{release_id}/{type_id}/{source_id}"))?;
    url.query_pairs_mut()
        .append_pair("sort", "0")
        .append_pair("token", "");
    let response: EpisodesResponse = client.get_json(url)?;
    ensure_success(response.code, "episodes")?;
    Ok(response.episodes)
}

fn ensure_success(code: i32, operation: &str) -> Result<(), ApiError> {
    if code == 0 {
        Ok(())
    } else {
        Err(ApiError::InvalidResponse(format!(
            "Anixart rejected the episode {operation} request (code {code})."
        )))
    }
}
