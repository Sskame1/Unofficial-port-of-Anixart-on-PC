use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseCard {
    pub id: u64,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub title_ru: Option<String>,
    #[serde(default)]
    pub title_original: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub year: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub creation_date: Option<i64>,
    #[serde(default)]
    pub aired_on_date: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub grade: Option<f64>,
    #[serde(default)]
    pub genres: Option<String>,
    #[serde(default)]
    pub episodes_released: Option<u32>,
    #[serde(default)]
    pub episodes_total: Option<u32>,
    #[serde(default)]
    pub category: Option<ReleaseCategory>,
    #[serde(default)]
    pub status: Option<ReleaseStatus>,
    #[serde(default)]
    pub age_rating: Option<u8>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub studio: Option<String>,
    #[serde(default)]
    pub duration: Option<u32>,
    #[serde(default)]
    pub season: Option<u32>,
    #[serde(default)]
    pub rating: Option<u64>,
    #[serde(default)]
    pub is_play_disabled: Option<bool>,
    #[serde(default)]
    pub is_view_blocked: Option<bool>,
    #[serde(default)]
    pub vote_count: Option<u64>,
    #[serde(default)]
    pub vote_1_count: Option<u64>,
    #[serde(default)]
    pub vote_2_count: Option<u64>,
    #[serde(default)]
    pub vote_3_count: Option<u64>,
    #[serde(default)]
    pub vote_4_count: Option<u64>,
    #[serde(default)]
    pub vote_5_count: Option<u64>,
    #[serde(default)]
    pub your_vote: Option<u8>,
    #[serde(default)]
    pub favorites_count: Option<u64>,
    #[serde(rename = "profile_list_status", default)]
    pub profile_list_status: Option<i32>,
    #[serde(rename = "is_favorite", default)]
    pub is_favorite: Option<bool>,
    #[serde(rename = "is_viewed", default)]
    pub is_viewed: Option<bool>,
    #[serde(default)]
    pub screenshot_images: Vec<String>,
    #[serde(default)]
    pub related_releases: Vec<ReleaseCard>,
    #[serde(default)]
    pub recommended_releases: Vec<ReleaseCard>,
    #[serde(default)]
    pub related: Option<RelatedReleaseGroup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelatedReleaseGroup {
    pub id: u64,
    #[serde(default)]
    pub name_ru: Option<String>,
    #[serde(default)]
    pub release_count: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseCategory {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseStatus {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InterestingItem {
    pub id: u64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type", default)]
    pub item_type: i32,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub is_hidden: bool,
}
