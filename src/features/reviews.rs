use crate::{
    api::{client::ApiClient, error::ApiError},
    features::discover::api::PageableResponse,
};
use serde_json::Value;

pub fn release_comments(
    client: &ApiClient,
    release_id: u64,
    page: u32,
    sort: u8,
    token: Option<&str>,
) -> Result<PageableResponse<Value>, ApiError> {
    let mut url = client.url(&format!(
        "release/comment/all/{release_id}/{}",
        page.saturating_sub(1)
    ))?;
    url.query_pairs_mut()
        .append_pair("sort", &sort.to_string())
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

pub fn vote_release_comment(
    client: &ApiClient,
    comment_id: u64,
    vote: i8,
    token: &str,
) -> Result<(), ApiError> {
    if !(1..=2).contains(&vote) {
        return Err(ApiError::InvalidResponse(
            "Comment vote must be 1 or 2.".to_owned(),
        ));
    }
    let mut url = client.url(&format!("release/comment/vote/{comment_id}/{vote}"))?;
    url.query_pairs_mut().append_pair("token", token);
    let response: Value = client.get_json(url)?;
    if response.get("code").and_then(Value::as_i64) != Some(0) {
        return Err(ApiError::InvalidResponse(
            "Anixart rejected comment vote.".to_owned(),
        ));
    }
    Ok(())
}
