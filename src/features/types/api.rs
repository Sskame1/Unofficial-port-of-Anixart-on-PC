use crate::api::{client::ApiClient, error::ApiError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentType {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Deserialize)]
struct TypesResponse {
    code: i32,
    #[serde(default)]
    types: Vec<ContentType>,
}

pub fn all(client: &ApiClient) -> Result<Vec<ContentType>, ApiError> {
    let mut url = client.url("type/all")?;
    url.query_pairs_mut().append_pair("token", "");
    let response: TypesResponse = client.get_json(url)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected type catalog request (code {}).",
            response.code
        )));
    }
    Ok(response.types)
}
