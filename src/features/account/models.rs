use crate::features::discover::models::ReleaseCard;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>", serialize = "T: Serialize"))]
pub struct Page<T> {
    pub code: i32,
    #[serde(default)]
    pub content: Vec<T>,
    #[serde(default)]
    pub current_page: u32,
    #[serde(default)]
    pub total_count: u64,
    #[serde(default)]
    pub total_page_count: u32,
}

pub type BookmarkPage = Page<ReleaseCard>;

pub type FeedPage = Page<serde_json::Value>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectionCard {
    pub id: u64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub creation_date: i64,
    #[serde(default)]
    pub last_update_date: i64,
    #[serde(default)]
    pub comment_count: u64,
    #[serde(default)]
    pub favorites_count: u64,
    #[serde(default)]
    pub is_favorite: bool,
    #[serde(default)]
    pub creator: Option<CollectionCreator>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectionCreator {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub avatar: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectionPage {
    pub code: i32,
    #[serde(default)]
    pub content: Vec<CollectionCard>,
    #[serde(default)]
    pub current_page: u32,
    #[serde(default)]
    pub total_count: u64,
    #[serde(default)]
    pub total_page_count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectionDetails {
    pub code: i32,
    #[serde(default)]
    pub collection: Option<CollectionCard>,
    #[serde(default)]
    pub releases: Vec<ReleaseCard>,
    #[serde(default)]
    pub comments: Vec<CollectionComment>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectionComment {
    pub id: u64,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub creation_date: i64,
    #[serde(default)]
    pub profile: Option<CollectionCreator>,
    #[serde(default)]
    pub vote: i32,
}
