pub mod api;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeType {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(rename = "is_sub", default)]
    pub is_sub: bool,
    #[serde(rename = "episodes_count", default)]
    pub episodes_count: u64,
    #[serde(rename = "view_count", default)]
    pub view_count: u64,
    #[serde(default)]
    pub workers: String,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub quality: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeSource {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(rename = "is_sub", default)]
    pub is_sub: bool,
    #[serde(rename = "episodes_count", default)]
    pub episodes_count: u64,
    #[serde(rename = "view_count", default)]
    pub view_count: u64,
    #[serde(rename = "type", default)]
    pub source_type: Option<EpisodeType>,
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
    #[serde(rename = "is_watched", default)]
    pub is_watched: bool,
    #[serde(rename = "playback_position", default)]
    pub playback_position: u64,
    #[serde(rename = "is_filler", default)]
    pub is_filler: bool,
}
