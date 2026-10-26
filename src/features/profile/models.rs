use crate::features::discover::models::ReleaseCard;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct ProfileResponse {
    pub code: i32,
    pub profile: Option<Profile>,
    #[serde(default)]
    pub is_my_profile: bool,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct Profile {
    pub id: u64,
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub rating_score: Option<f64>,
    #[serde(default)]
    pub added_date: Option<i64>,
    #[serde(default)]
    pub register_date: Option<i64>,
    #[serde(default)]
    pub watching_count: u32,
    #[serde(default)]
    pub plan_count: u32,
    #[serde(default)]
    pub completed_count: u32,
    #[serde(default)]
    pub hold_on_count: u32,
    #[serde(default)]
    pub dropped_count: u32,
    #[serde(default)]
    pub watched_episode_count: u32,
    #[serde(default)]
    pub watched_time: u64,
    #[serde(default)]
    pub friend_count: u32,
    #[serde(default)]
    pub watch_dynamics: Vec<WatchDynamic>,
    #[serde(default)]
    pub history: Vec<ReleaseCard>,
    #[serde(default)]
    pub is_counts_hidden: bool,
    #[serde(default)]
    pub is_stats_hidden: bool,
    #[serde(default)]
    pub is_private: bool,
    #[serde(default)]
    pub is_verified: bool,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct ProfileFriend {
    pub id: u64,
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub is_verified: bool,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct FriendsPage {
    pub code: i32,
    #[serde(default)]
    pub content: Vec<ProfileFriend>,
    #[serde(default)]
    pub current_page: u32,
    #[serde(default)]
    pub total_page_count: u32,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct WatchDynamic {
    #[serde(default)]
    pub day: Option<i64>,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub count: u32,
}
