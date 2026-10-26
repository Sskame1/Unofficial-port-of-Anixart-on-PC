use crate::{
    api::{client::ApiClient, error::ApiError},
    features::discover::api::PageableResponse,
};
use serde_json::Value;

pub fn release_comments(
    client: &ApiClient,
    release_id: u64,
    page: u32,
    token: Option<&str>,
) -> Result<PageableResponse<Value>, ApiError> {
    let mut url = client.url(&format!(
        "release/comment/all/{release_id}/{}",
        page.saturating_sub(1)
    ))?;
    url.query_pairs_mut()
        .append_pair("sort", "0")
        .append_pair("token", token.unwrap_or(""));
    let response: PageableResponse<Value> = client.get_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected release comments request (code {}).",
            response.code
        )));
    }
    Ok(response)
}
