pub mod api;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub fn string_or_default<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer).map(|value| value.unwrap_or_default())
}

pub fn u64_or_default<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<u64>::deserialize(deserializer).map(|value| value.unwrap_or_default())
}

pub fn u32_or_default<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<u32>::deserialize(deserializer).map(|value| value.unwrap_or_default())
}

pub fn bool_or_default<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<bool>::deserialize(deserializer).map(|value| value.unwrap_or_default())
}

fn episode_type_or_id<'de, D>(deserializer: D) -> Result<Option<EpisodeType>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<Value>::deserialize(deserializer)? {
        Some(Value::Object(value)) => serde_json::from_value(Value::Object(value))
            .map(Some)
            .map_err(serde::de::Error::custom),
        // Some API responses serialize Source.type as a numeric reference;
        // the source row already contains its own name, so no nested details
        // are needed to render it.
        Some(Value::Number(_)) | Some(Value::String(_)) | None | Some(Value::Null) => Ok(None),
        Some(other) => Err(serde::de::Error::custom(format!(
            "expected episode type object or ID, found {other}"
        ))),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeType {
    pub id: u64,
    #[serde(default, deserialize_with = "string_or_default")]
    pub name: String,
    #[serde(default, deserialize_with = "string_or_default")]
    pub icon: String,
    #[serde(rename = "is_sub", default, deserialize_with = "bool_or_default")]
    pub is_sub: bool,
    #[serde(
        rename = "episodes_count",
        default,
        deserialize_with = "u64_or_default"
    )]
    pub episodes_count: u64,
    #[serde(rename = "view_count", default, deserialize_with = "u64_or_default")]
    pub view_count: u64,
    #[serde(default, deserialize_with = "string_or_default")]
    pub workers: String,
    #[serde(default, deserialize_with = "bool_or_default")]
    pub pinned: bool,
    #[serde(default, deserialize_with = "u32_or_default")]
    pub quality: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeSource {
    pub id: u64,
    #[serde(default, deserialize_with = "string_or_default")]
    pub name: String,
    #[serde(default, deserialize_with = "string_or_default")]
    pub icon: String,
    #[serde(rename = "is_sub", default, deserialize_with = "bool_or_default")]
    pub is_sub: bool,
    #[serde(
        rename = "episodes_count",
        default,
        deserialize_with = "u64_or_default"
    )]
    pub episodes_count: u64,
    #[serde(rename = "view_count", default, deserialize_with = "u64_or_default")]
    pub view_count: u64,
    #[serde(rename = "type", default, deserialize_with = "episode_type_or_id")]
    pub source_type: Option<EpisodeType>,
    #[serde(default, deserialize_with = "u32_or_default")]
    pub quality: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodeItem {
    pub position: u32,
    #[serde(default, deserialize_with = "string_or_default")]
    pub name: String,
    #[serde(default, deserialize_with = "string_or_default")]
    pub url: String,
    #[serde(default, deserialize_with = "bool_or_default")]
    pub iframe: bool,
    #[serde(default, deserialize_with = "u32_or_default")]
    pub quality: u32,
    #[serde(rename = "is_watched", default, deserialize_with = "bool_or_default")]
    pub is_watched: bool,
    #[serde(
        rename = "playback_position",
        default,
        deserialize_with = "u64_or_default"
    )]
    pub playback_position: u64,
    #[serde(rename = "is_filler", default, deserialize_with = "bool_or_default")]
    pub is_filler: bool,
}
