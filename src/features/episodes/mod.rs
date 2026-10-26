pub mod api;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeType {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub quality: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeSource {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub quality: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeItem {
    pub position: u32,
    pub name: String,
    pub url: String,
    pub iframe: bool,
    #[serde(default)]
    pub quality: u32,
}
