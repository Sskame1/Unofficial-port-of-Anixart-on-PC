use crate::{
    api::{client::ApiClient, error::ApiError},
    features::discover::models::ReleaseCard,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SearchRequest<'a> {
    query: &'a str,
    search_by: u8,
}

#[derive(Deserialize)]
struct SearchResponse {
    code: i32,
    #[serde(default)]
    releases: Vec<ReleaseCard>,
}

pub fn search_releases(client: &ApiClient, query: &str) -> Result<Vec<ReleaseCard>, ApiError> {
    let mut url = client.url("search/releases/0")?;
    url.query_pairs_mut().append_pair("token", "");

    let request = SearchRequest {
        query,
        search_by: 0,
    };
    let response: SearchResponse =
        client.post_json_with_header(url, &request, "API-Version", "v2")?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected the release search (code {}).",
            response.code
        )));
    }
    Ok(response.releases)
}
