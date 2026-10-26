use crate::{
    api::{client::ApiClient, error::ApiError},
    features::discover::models::{InterestingItem, ReleaseCard},
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
    token: Option<&str>,
) -> Result<PageableResponse<ReleaseCard>, ApiError> {
    if matches!(kind, DiscoverKind::Interesting) {
        let response = interesting(client)?;
        return Ok(PageableResponse {
            code: response.code,
            content: response
                .content
                .into_iter()
                .map(|item| ReleaseCard {
                    id: item.id,
                    action: Some(item.action),
                    title_ru: Some(item.title),
                    title_original: None,
                    image: Some(item.image),
                    year: None,
                    description: Some(item.description),
                    grade: None,
                    genres: None,
                    episodes_released: None,
                    episodes_total: None,
                    category: None,
                    status: None,
                    age_rating: None,
                    source: None,
                    country: None,
                    studio: None,
                    duration: None,
                    season: None,
                    rating: None,
                    is_play_disabled: None,
                    is_view_blocked: None,
                    vote_count: None,
                    favorites_count: None,
                    profile_list_status: None,
                    is_favorite: None,
                    is_viewed: None,
                    screenshot_images: Vec::new(),
                    related_releases: Vec::new(),
                    recommended_releases: Vec::new(),
                })
                .collect(),
            current_page: response.current_page,
            total_count: response.total_count,
            total_page_count: response.total_page_count,
        });
    }
    let page = page.saturating_sub(1);
    let path = match kind {
        DiscoverKind::Recommendations => format!("discover/recommendations/{page}"),
        DiscoverKind::Discussing => "discover/discussing".to_owned(),
        DiscoverKind::Watching => format!("discover/watching/{page}"),
        DiscoverKind::Interesting => unreachable!(),
    };
    let mut url = client.url(&path)?;
    url.query_pairs_mut()
        .append_pair("token", token.unwrap_or(""));
    if matches!(kind, DiscoverKind::Recommendations) {
        let previous_page = if page > 0 { -1 } else { -1 };
        url.query_pairs_mut()
            .append_pair("previous_page", &previous_page.to_string());
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

pub fn interesting(client: &ApiClient) -> Result<PageableResponse<InterestingItem>, ApiError> {
    let mut url = client.url("discover/interesting")?;
    url.query_pairs_mut().append_pair("token", "");
    let response: PageableResponse<InterestingItem> = client.post_empty_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected interesting request (code {}).",
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
