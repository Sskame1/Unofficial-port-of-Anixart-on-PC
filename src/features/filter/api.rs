use crate::{
    api::{client::ApiClient, error::ApiError},
    features::discover::models::ReleaseCard,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterRequest {
    #[serde(rename = "category_id", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<u64>,
    #[serde(rename = "status_id", skip_serializing_if = "Option::is_none")]
    pub status_id: Option<u64>,
    #[serde(rename = "start_year", skip_serializing_if = "Option::is_none")]
    pub start_year: Option<i32>,
    #[serde(rename = "end_year", skip_serializing_if = "Option::is_none")]
    pub end_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "episodes_from", skip_serializing_if = "Option::is_none")]
    pub episodes_from: Option<i32>,
    #[serde(rename = "episodes_to", skip_serializing_if = "Option::is_none")]
    pub episodes_to: Option<i32>,
    #[serde(default)]
    pub sort: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<i32>,
    #[serde(
        rename = "episode_duration_from",
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_duration_from: Option<i32>,
    #[serde(
        rename = "episode_duration_to",
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_duration_to: Option<i32>,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(rename = "profile_list_exclusions", default)]
    pub profile_list_exclusions: Vec<i32>,
    #[serde(default)]
    pub types: Vec<u64>,
    #[serde(rename = "age_ratings", default)]
    pub age_ratings: Vec<i32>,
    #[serde(rename = "is_genres_exclude_mode_enabled", default)]
    pub is_genres_exclude_mode_enabled: bool,
    #[serde(rename = "genres_mode", skip_serializing_if = "Option::is_none")]
    pub genres_mode: Option<i32>,
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

pub fn search(
    client: &ApiClient,
    page: u32,
    request: &FilterRequest,
) -> Result<PageableResponse<ReleaseCard>, ApiError> {
    let mut url = client.url(&format!("filter/{page}"))?;
    url.query_pairs_mut()
        .append_pair("extended_mode", "true")
        .append_pair("token", "");
    let response: PageableResponse<ReleaseCard> =
        client.post_json_with_header(url, request, "", "")?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected filter request (code {}).",
            response.code
        )));
    }
    Ok(response)
}
