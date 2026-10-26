use crate::{
    api::{client::ApiClient, error::ApiError},
    features::discover::models::ReleaseCard,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleResponse {
    pub code: i32,
    #[serde(default)]
    pub monday: Vec<ReleaseCard>,
    #[serde(default)]
    pub tuesday: Vec<ReleaseCard>,
    #[serde(default)]
    pub wednesday: Vec<ReleaseCard>,
    #[serde(default)]
    pub thursday: Vec<ReleaseCard>,
    #[serde(default)]
    pub friday: Vec<ReleaseCard>,
    #[serde(default)]
    pub saturday: Vec<ReleaseCard>,
    #[serde(default)]
    pub sunday: Vec<ReleaseCard>,
}

pub fn week(client: &ApiClient) -> Result<ScheduleResponse, ApiError> {
    let response: ScheduleResponse = client.get_json(client.url("schedule")?)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected schedule request (code {}).",
            response.code
        )));
    }
    Ok(response)
}
