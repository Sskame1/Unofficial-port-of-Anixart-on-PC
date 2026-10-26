#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInRequest {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub code: i32,
    #[serde(default)]
    pub profile: Option<ProfileSummary>,
    #[serde(default)]
    pub profile_token: Option<ProfileToken>,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileSummary {
    pub id: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileToken {
    #[serde(default)]
    pub token: String,
}
