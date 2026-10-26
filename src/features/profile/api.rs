use crate::{
    api::{client::ApiClient, error::ApiError},
    features::profile::models::{FriendsPage, Profile, ProfileResponse},
};

pub fn info(client: &ApiClient, id: u64, token: &str) -> Result<ProfileResponse, ApiError> {
    let mut url = client.url(&format!("profile/{id}"))?;
    url.query_pairs_mut().append_pair("token", token);
    let response: ProfileResponse = client.get_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected profile request (code {}).",
            response.code
        )));
    }
    Ok(response)
}

pub fn profile(client: &ApiClient, id: u64, token: &str) -> Result<Profile, ApiError> {
    info(client, id, token)?
        .profile
        .ok_or_else(|| ApiError::InvalidResponse("Anixart returned no profile data.".to_owned()))
}

pub fn friends(
    client: &ApiClient,
    id: u64,
    page: u32,
    token: &str,
) -> Result<FriendsPage, ApiError> {
    let mut url = client.url(&format!(
        "profile/friend/all/{id}/{}",
        page.saturating_sub(1)
    ))?;
    url.query_pairs_mut().append_pair("token", token);
    let response: FriendsPage = client.get_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected friends request (code {}).",
            response.code
        )));
    }
    Ok(response)
}
