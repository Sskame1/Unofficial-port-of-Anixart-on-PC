use crate::{
    api::{client::ApiClient, error::ApiError},
    features::discover::models::ReleaseCard,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum DiscoverKind {
    Recommendations,
    Discussing,
    Watching,
    Interesting,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>", serialize = "T: Serialize"))]
pub struct PageableResponse<T> {
    pub code: i32,
    #[serde(default)]
    pub content: Vec<T>,
    #[serde(default)]
    pub current_page: u32,
    #[serde(default)]
    pub total_count: u64,
    #[serde(default)]
    pub total_page_count: u32,
}

pub fn page(
    client: &ApiClient,
    kind: DiscoverKind,
    page: u32,
) -> Result<PageableResponse<ReleaseCard>, ApiError> {
    let page = page.max(1);
    let path = match kind {
        DiscoverKind::Recommendations => format!("discover/recommendations/{page}"),
        DiscoverKind::Discussing => "discover/discussing".to_owned(),
        DiscoverKind::Watching => format!("discover/watching/{page}"),
        DiscoverKind::Interesting => "discover/interesting".to_owned(),
    };
    let mut url = client.url(&path)?;
    url.query_pairs_mut().append_pair("token", "");
    if matches!(kind, DiscoverKind::Recommendations) {
        url.query_pairs_mut()
            .append_pair("previous_page", &(page.saturating_sub(1)).to_string());
    }
    let response: PageableResponse<ReleaseCard> = client.post_empty_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected discover request (code {}).",
            response.code
        )));
    }
    Ok(response)
}

#[derive(Deserialize)]
struct ReleaseResponse {
    code: i32,
    release: Option<ReleaseCard>,
}

pub fn random_release(client: &ApiClient) -> Result<ReleaseCard, ApiError> {
    let mut url = client.url("release/random")?;
    url.query_pairs_mut()
        .append_pair("extended_mode", "true")
        .append_pair("token", "");

    let response: ReleaseResponse = client.get_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected the random release request (code {}).",
            response.code
        )));
    }

    response
        .release
        .ok_or_else(|| ApiError::InvalidResponse("Anixart returned no release.".to_owned()))
}

pub fn release(client: &ApiClient, release_id: u64) -> Result<ReleaseCard, ApiError> {
    let mut url = client.url(&format!("release/{release_id}"))?;
    url.query_pairs_mut()
        .append_pair("extended_mode", "true")
        .append_pair("token", "");
    let response: ReleaseResponse = client.get_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected the release request (code {}).",
            response.code
        )));
    }
    response
        .release
        .ok_or_else(|| ApiError::InvalidResponse("Anixart returned no release.".to_owned()))
}
