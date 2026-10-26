use crate::{
    api::{client::ApiClient, error::ApiError},
    features::account::models::{
        BookmarkPage, CollectionComment, CollectionDetails, CollectionPage, FeedPage, Page,
    },
};

fn ensure(code: i32, name: &str) -> Result<(), ApiError> {
    if code == 0 {
        Ok(())
    } else {
        Err(ApiError::InvalidResponse(format!(
            "Anixart rejected {name} request (code {code})."
        )))
    }
}

pub fn bookmarks(
    client: &ApiClient,
    token: &str,
    kind: &str,
    page: u32,
) -> Result<BookmarkPage, ApiError> {
    let page = page.saturating_sub(1);
    let path = match kind {
        "favorites" => format!("favorite/all/{page}"),
        "watching" => format!("profile/list/all/1/{page}"),
        "planned" => format!("profile/list/all/2/{page}"),
        "completed" => format!("profile/list/all/3/{page}"),
        "postponed" => format!("profile/list/all/4/{page}"),
        "dropped" => format!("profile/list/all/5/{page}"),
        _ => {
            return Err(ApiError::InvalidResponse(
                "Unknown bookmark category.".to_owned(),
            ));
        }
    };
    let mut url = client.url(&path)?;
    url.query_pairs_mut()
        .append_pair("sort", "1")
        .append_pair("filter_announce", "0")
        .append_pair("token", token);
    let response: BookmarkPage = client.get_json(url)?;
    ensure(response.code, "bookmarks")?;
    Ok(response)
}

pub fn feed(client: &ApiClient, token: &str, page: u32) -> Result<FeedPage, ApiError> {
    let mut url = client.url(&format!("feed/all/{}", page.saturating_sub(1)))?;
    url.query_pairs_mut()
        .append_pair("date", "0")
        .append_pair("token", token);
    let response: FeedPage = match client.get_json(url) {
        Ok(response) => response,
        Err(ApiError::HttpStatus(status)) if status == reqwest::StatusCode::BAD_REQUEST => {
            let mut fallback = client.url(&format!("feed/my/all/{}", page.saturating_sub(1)))?;
            fallback.query_pairs_mut().append_pair("token", token);
            client.get_json(fallback)?
        }
        Err(error) => return Err(error),
    };
    ensure(response.code, "feed")?;
    Ok(response)
}

pub fn history(client: &ApiClient, token: &str, page: u32) -> Result<BookmarkPage, ApiError> {
    let mut url = client.url(&format!("history/{}", page.saturating_sub(1)))?;
    url.query_pairs_mut().append_pair("token", token);
    let response: BookmarkPage = client.get_json(url)?;
    ensure(response.code, "history")?;
    Ok(response)
}

pub fn collections(
    client: &ApiClient,
    token: Option<&str>,
    kind: &str,
    page: u32,
    profile_id: Option<u64>,
) -> Result<CollectionPage, ApiError> {
    let page = page.saturating_sub(1);
    let token = token.unwrap_or("");
    let mut url = if kind == "mine" {
        let id = profile_id
            .ok_or_else(|| ApiError::InvalidResponse("Профиль не авторизован.".to_owned()))?;
        client.url(&format!("collection/all/profile/{id}/{page}"))?
    } else {
        client.url(&format!("collection/all/{page}"))?
    };
    url.query_pairs_mut().append_pair("token", token);
    if kind != "mine" {
        let sort = match kind {
            "new" => 5,
            _ => 4,
        };
        url.query_pairs_mut()
            .append_pair("previous_page", "-1")
            .append_pair("where", "2")
            .append_pair("sort", &sort.to_string());
    }
    let response: CollectionPage = client.get_json(url)?;
    ensure(response.code, "collections")?;
    Ok(response)
}

pub fn collection(
    client: &ApiClient,
    token: Option<&str>,
    id: u64,
) -> Result<CollectionDetails, ApiError> {
    let mut url = client.url(&format!("collection/{id}"))?;
    url.query_pairs_mut()
        .append_pair("token", token.unwrap_or(""));
    let details: CollectionDetails = client.get_json(url)?;
    ensure(details.code, "collection")?;
    let mut releases_url = client.url(&format!("collection/{id}/releases/0"))?;
    releases_url
        .query_pairs_mut()
        .append_pair("token", token.unwrap_or(""));
    let releases: crate::features::discover::api::PageableResponse<
        crate::features::discover::models::ReleaseCard,
    > = client.get_json(releases_url)?;
    ensure(releases.code, "collection releases")?;
    let mut comments_url = client.url(&format!("collection/comment/all/{id}/0"))?;
    comments_url
        .query_pairs_mut()
        .append_pair("sort", "1")
        .append_pair("token", token.unwrap_or(""));
    let comments = client
        .get_json::<Page<CollectionComment>>(comments_url)
        .map(|page| page.content)
        .unwrap_or_default();
    Ok(CollectionDetails {
        releases: releases.content,
        comments,
        ..details
    })
}

pub fn toggle_collection_favorite(
    client: &ApiClient,
    token: &str,
    id: u64,
    enabled: bool,
) -> Result<(), ApiError> {
    let path = if enabled {
        format!("collectionFavorite/add/{id}")
    } else {
        format!("collectionFavorite/delete/{id}")
    };
    let mut url = client.url(&path)?;
    url.query_pairs_mut().append_pair("token", token);
    let response: serde_json::Value = client.get_json(url)?;
    let code = response
        .get("code")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(0);
    ensure(code as i32, "collection favorite")
}

pub fn add_release_to_list(
    client: &ApiClient,
    token: &str,
    release_id: u64,
    list_type: u8,
) -> Result<(), ApiError> {
    let mut url = client.url(&format!("profile/list/add/{list_type}/{release_id}"))?;
    url.query_pairs_mut().append_pair("token", token);
    let response: serde_json::Value = client.get_json(url)?;
    let code = response
        .get("code")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(-1);
    ensure(code as i32, "add release to list")
}

pub fn article(
    client: &ApiClient,
    token: Option<&str>,
    id: u64,
) -> Result<serde_json::Value, ApiError> {
    let mut url = client.url(&format!("article/{id}"))?;
    url.query_pairs_mut()
        .append_pair("token", token.unwrap_or(""));
    let response: serde_json::Value = client.get_json(url)?;
    let code = response
        .get("code")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(-1);
    if code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected article request (code {code})."
        )));
    }
    Ok(response.get("article").cloned().unwrap_or(response))
}
