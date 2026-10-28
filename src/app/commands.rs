use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AppCommand {
    Navigate {
        route: String,
    },
    PlayEpisode {
        release_id: u64,
        type_id: u64,
        source_id: u64,
        position: u32,
    },
    WatchEpisode {
        release_id: u64,
        source_id: u64,
        position: u32,
    },
    SaveEpisodeProgress {
        release_id: u64,
        type_id: u64,
        source_id: u64,
        position: u32,
        playback_position: u64,
        watched: bool,
    },
    SetImageCacheEnabled { enabled: bool },
    CacheImage { request_id: u64, url: String, category: String },
    DiscoverRandomRelease,
    OpenRelease {
        release_id: u64,
    },
    LoadEpisodeTypes {
        release_id: u64,
    },
    LoadEpisodeSources {
        release_id: u64,
        type_id: u64,
    },
    LoadEpisodes {
        release_id: u64,
        type_id: u64,
        source_id: u64,
    },
    SearchReleases {
        query: String,
    },
    SignIn {
        login: String,
        password: String,
    },
    SignInGoogle,
    SignOut,
    LoadBookmarks {
        kind: String,
        page: u32,
    },
    LoadHistory {
        page: u32,
    },
    LoadFeed {
        #[serde(default = "default_feed_kind")]
        kind: String,
        page: u32,
        request_id: u64,
    },
    LoadCollections {
        kind: String,
        page: u32,
        request_id: u64,
    },
    OpenCollection {
        collection_id: u64,
        request_id: u64,
    },
    ToggleCollectionFavorite {
        collection_id: u64,
        enabled: bool,
    },
    AddReleaseToList {
        release_id: u64,
        list_type: u8,
        current_status: Option<i32>,
    },
    ToggleReleaseFavorite {
        release_id: u64,
        enabled: bool,
    },
    VoteRelease {
        release_id: u64,
        vote: u8,
    },
    LoadFriends {
        page: u32,
    },
    RefreshProfile,
    LoadReleaseComments {
        release_id: u64,
        page: u32,
        sort: u8,
    },
    VoteReleaseComment {
        comment_id: u64,
        vote: i8,
    },
    LoadArticle {
        article_id: u64,
    },
    LoadDiscoverPage {
        kind: String,
        page: u32,
    },
    LoadSchedule,
    LoadTypes,
    ApplyFilter {
        page: u32,
        filter: crate::features::filter::api::FilterRequest,
    },
}

#[derive(Debug)]
pub enum AppEvent {
    Navigate(String),
    PlayEpisode {
        release_id: u64,
        type_id: u64,
        source_id: u64,
        position: u32,
    },
    WatchEpisode {
        release_id: u64,
        source_id: u64,
        position: u32,
    },
    SaveEpisodeProgress {
        release_id: u64,
        type_id: u64,
        source_id: u64,
        position: u32,
        playback_position: u64,
        watched: bool,
    },
    EpisodeWatchResolved(Result<(), String>),
    SetImageCacheEnabled(bool),
    CacheImage { request_id: u64, url: String, category: String },
    CachedImageResolved { request_id: u64, result: Result<String, String> },
    DiscoverRandomRelease,
    OpenRelease(u64),
    RandomReleaseResolved(Result<crate::features::discover::models::ReleaseCard, String>),
    ReleaseResolved(Result<crate::features::discover::models::ReleaseCard, String>),
    LoadEpisodeTypes(u64),
    EpisodeTypesResolved(Result<Vec<crate::features::episodes::EpisodeType>, String>),
    LoadEpisodeSources {
        release_id: u64,
        type_id: u64,
    },
    EpisodeSourcesResolved(Result<Vec<crate::features::episodes::EpisodeSource>, String>),
    LoadEpisodes {
        release_id: u64,
        type_id: u64,
        source_id: u64,
    },
    EpisodesResolved(Result<Vec<crate::features::episodes::EpisodeItem>, String>),
    SearchReleases(String),
    SignIn {
        login: String,
        password: String,
    },
    SignInGoogle,
    SignOut,
    SearchResolved(Result<Vec<crate::features::discover::models::ReleaseCard>, String>),
    AuthResolved(Result<crate::features::auth::session::AuthSession, String>),
    ProfileResolved(Result<crate::features::profile::models::Profile, String>),
    LoadBookmarks {
        kind: String,
        page: u32,
    },
    BookmarksResolved(Result<crate::features::account::models::BookmarkPage, String>),
    LoadHistory {
        page: u32,
    },
    HistoryResolved(Result<crate::features::account::models::BookmarkPage, String>),
    LoadFeed {
        kind: String,
        page: u32,
        request_id: u64,
    },
    FeedResolved {
        request_id: u64,
        result: Result<crate::features::account::models::FeedPage, String>,
    },
    LoadCollections {
        kind: String,
        page: u32,
        request_id: u64,
    },
    CollectionsResolved {
        request_id: u64,
        result: Result<crate::features::account::models::CollectionPage, String>,
    },
    OpenCollection {
        collection_id: u64,
        request_id: u64,
    },
    CollectionResolved {
        request_id: u64,
        result: Result<crate::features::account::models::CollectionDetails, String>,
    },
    ToggleCollectionFavorite {
        collection_id: u64,
        enabled: bool,
    },
    AddReleaseToList {
        release_id: u64,
        list_type: u8,
        current_status: Option<i32>,
    },
    ToggleReleaseFavorite {
        release_id: u64,
        enabled: bool,
    },
    VoteRelease {
        release_id: u64,
        vote: u8,
    },
    CollectionFavoriteResolved(Result<bool, String>),
    ReleaseListResolved(Result<(), String>),
    ReleaseFavoriteResolved(Result<bool, String>),
    ReleaseVoteResolved(Result<u8, String>),
    LoadFriends {
        page: u32,
    },
    RefreshProfile,
    FriendsResolved(Result<crate::features::profile::models::FriendsPage, String>),
    LoadReleaseComments {
        release_id: u64,
        page: u32,
        sort: u8,
    },
    ReleaseCommentsResolved(Result<serde_json::Value, String>),
    VoteReleaseComment {
        comment_id: u64,
        vote: i8,
    },
    ReleaseCommentVoteResolved(Result<(), String>),
    LoadArticle(u64),
    ArticleResolved(Result<serde_json::Value, String>),
    LoadDiscoverPage {
        kind: String,
        page: u32,
    },
    DiscoverPageResolved {
        kind: String,
        result: Result<
            crate::features::discover::api::PageableResponse<
                crate::features::discover::models::ReleaseCard,
            >,
            String,
        >,
    },
    LoadSchedule,
    ScheduleResolved(Result<crate::features::schedule::api::ScheduleResponse, String>),
    LoadTypes,
    TypesResolved(Result<Vec<crate::features::types::ContentType>, String>),
    ApplyFilter {
        page: u32,
        filter: crate::features::filter::api::FilterRequest,
    },
    FilterResolved(
        Result<
            crate::features::filter::api::PageableResponse<
                crate::features::discover::models::ReleaseCard,
            >,
            String,
        >,
    ),
    EpisodeResolved(Result<crate::features::player::resolver::EpisodeLink, String>),
    Alert(String),
}

impl AppCommand {
    pub fn into_event(self) -> Result<AppEvent, String> {
        match self {
            Self::Navigate { route } => {
                if crate::app::state::AppState::is_known_route(&route) {
                    Ok(AppEvent::Navigate(route))
                } else {
                    Err(format!("Unknown section: {route}"))
                }
            }
            Self::PlayEpisode {
                release_id,
                type_id,
                source_id,
                position,
            } => Ok(AppEvent::PlayEpisode {
                release_id,
                type_id,
                source_id,
                position,
            }),
            Self::WatchEpisode {
                release_id,
                source_id,
                position,
            } if release_id > 0 && source_id > 0 && position > 0 => Ok(AppEvent::WatchEpisode {
                release_id,
                source_id,
                position,
            }),
            Self::WatchEpisode { .. } => {
                Err("ID релиза, источника и серии должны быть больше нуля.".to_owned())
            }
            Self::SaveEpisodeProgress {
                release_id,
                type_id,
                source_id,
                position,
                playback_position,
                watched,
            } if release_id > 0 && type_id > 0 && source_id > 0 && position > 0 => Ok(AppEvent::SaveEpisodeProgress {
                release_id,
                type_id,
                source_id,
                position,
                playback_position,
                watched,
            }),
            Self::SaveEpisodeProgress { .. } => {
                Err("Параметры прогресса серии указаны неверно.".to_owned())
            }
            Self::SetImageCacheEnabled { enabled } => Ok(AppEvent::SetImageCacheEnabled(enabled)),
            Self::CacheImage { request_id, url, category }
                if request_id > 0 && url.len() <= 4096 && url.starts_with("https://") =>
            {
                Ok(AppEvent::CacheImage { request_id, url, category })
            }
            Self::CacheImage { .. } => Err("Некорректный запрос изображения в кеш.".to_owned()),
            Self::DiscoverRandomRelease => Ok(AppEvent::DiscoverRandomRelease),
            Self::OpenRelease { release_id } if release_id > 0 => {
                Ok(AppEvent::OpenRelease(release_id))
            }
            Self::OpenRelease { .. } => Err("ID релиза должен быть больше нуля.".to_owned()),
            Self::LoadEpisodeTypes { release_id } if release_id > 0 => {
                Ok(AppEvent::LoadEpisodeTypes(release_id))
            }
            Self::LoadEpisodeTypes { .. } => Err("ID релиза должен быть больше нуля.".to_owned()),
            Self::LoadEpisodeSources {
                release_id,
                type_id,
            } if release_id > 0 && type_id > 0 => Ok(AppEvent::LoadEpisodeSources {
                release_id,
                type_id,
            }),
            Self::LoadEpisodeSources { .. } => {
                Err("ID релиза и типа должны быть больше нуля.".to_owned())
            }
            Self::LoadEpisodes {
                release_id,
                type_id,
                source_id,
            } if release_id > 0 && type_id > 0 && source_id > 0 => Ok(AppEvent::LoadEpisodes {
                release_id,
                type_id,
                source_id,
            }),
            Self::LoadEpisodes { .. } => {
                Err("ID релиза, типа и источника должны быть больше нуля.".to_owned())
            }
            Self::SearchReleases { query } => {
                let query = query.trim().to_owned();
                if query.is_empty() {
                    return Err("Введите название релиза для поиска.".to_owned());
                }
                if query.chars().count() > 120 {
                    return Err("Запрос слишком длинный (максимум 120 символов).".to_owned());
                }
                Ok(AppEvent::SearchReleases(query))
            }
            Self::SignIn { login, password } => {
                let login = login.trim().to_owned();
                if login.is_empty() || password.is_empty() {
                    return Err("Введите логин и пароль.".to_owned());
                }
                Ok(AppEvent::SignIn { login, password })
            }
            Self::SignInGoogle => Ok(AppEvent::SignInGoogle),
            Self::SignOut => Ok(AppEvent::SignOut),
            Self::LoadBookmarks { kind, page }
                if page > 0
                    && matches!(
                        kind.as_str(),
                        "favorites"
                            | "watching"
                            | "planned"
                            | "completed"
                            | "postponed"
                            | "dropped"
                    ) =>
            {
                Ok(AppEvent::LoadBookmarks { kind, page })
            }
            Self::LoadBookmarks { .. } => {
                Err("Категория закладок или страница указана неверно.".to_owned())
            }
            Self::LoadHistory { page } if page > 0 => Ok(AppEvent::LoadHistory { page }),
            Self::LoadHistory { .. } => {
                Err("Номер страницы истории должен быть больше нуля.".to_owned())
            }
            Self::LoadFeed { kind, page, request_id }
                if page > 0 && matches!(kind.as_str(), "mine" | "latest") =>
            {
                Ok(AppEvent::LoadFeed { kind, page, request_id })
            }
            Self::LoadFeed { .. } => {
                Err("Номер страницы ленты должен быть больше нуля.".to_owned())
            }
            Self::LoadCollections {
                kind,
                page,
                request_id,
            } if page > 0 && matches!(kind.as_str(), "popular" | "new" | "mine") => {
                Ok(AppEvent::LoadCollections {
                    kind,
                    page,
                    request_id,
                })
            }
            Self::LoadCollections { .. } => {
                Err("Раздел коллекций или страница указаны неверно.".to_owned())
            }
            Self::OpenCollection {
                collection_id,
                request_id,
            } if collection_id > 0 => Ok(AppEvent::OpenCollection {
                collection_id,
                request_id,
            }),
            Self::OpenCollection { .. } => Err("ID коллекции должен быть больше нуля.".to_owned()),
            Self::ToggleCollectionFavorite {
                collection_id,
                enabled,
            } if collection_id > 0 => Ok(AppEvent::ToggleCollectionFavorite {
                collection_id,
                enabled,
            }),
            Self::ToggleCollectionFavorite { .. } => {
                Err("ID коллекции должен быть больше нуля.".to_owned())
            }
            Self::AddReleaseToList {
                release_id,
                list_type,
                current_status,
            } if release_id > 0 && (1..=5).contains(&list_type) => Ok(AppEvent::AddReleaseToList {
                release_id,
                list_type,
                current_status,
            }),
            Self::AddReleaseToList { .. } => Err("Релиз или тип списка указан неверно.".to_owned()),
            Self::ToggleReleaseFavorite {
                release_id,
                enabled,
            } if release_id > 0 => Ok(AppEvent::ToggleReleaseFavorite {
                release_id,
                enabled,
            }),
            Self::ToggleReleaseFavorite { .. } => {
                Err("ID релиза должен быть больше нуля.".to_owned())
            }
            Self::VoteRelease { release_id, vote } if release_id > 0 && vote <= 5 => {
                Ok(AppEvent::VoteRelease { release_id, vote })
            }
            Self::VoteRelease { .. } => Err("Релиз или оценка указаны неверно.".to_owned()),
            Self::LoadFriends { page } if page > 0 => Ok(AppEvent::LoadFriends { page }),
            Self::LoadFriends { .. } => {
                Err("Номер страницы друзей должен быть больше нуля.".to_owned())
            }
            Self::RefreshProfile => Ok(AppEvent::RefreshProfile),
            Self::LoadReleaseComments {
                release_id,
                page,
                sort,
            } if release_id > 0 && page > 0 && (1..=3).contains(&sort) => {
                Ok(AppEvent::LoadReleaseComments {
                    release_id,
                    page,
                    sort,
                })
            }
            Self::LoadReleaseComments { .. } => {
                Err("ID релиза или страница отзывов указаны неверно.".to_owned())
            }
            Self::VoteReleaseComment { comment_id, vote }
                if comment_id > 0 && (1..=2).contains(&vote) =>
            {
                Ok(AppEvent::VoteReleaseComment { comment_id, vote })
            }
            Self::VoteReleaseComment { .. } => {
                Err("Комментарий или оценка указаны неверно.".to_owned())
            }
            Self::LoadArticle { article_id } if article_id > 0 => {
                Ok(AppEvent::LoadArticle(article_id))
            }
            Self::LoadArticle { .. } => Err("ID публикации должен быть больше нуля.".to_owned()),
            Self::LoadDiscoverPage { kind, page }
                if page > 0
                    && matches!(
                        kind.as_str(),
                        "recommendations" | "discussing" | "watching" | "interesting"
                    ) =>
            {
                Ok(AppEvent::LoadDiscoverPage { kind, page })
            }
            Self::LoadDiscoverPage { .. } => {
                Err("Раздел discover или номер страницы указан неверно.".to_owned())
            }
            Self::LoadSchedule => Ok(AppEvent::LoadSchedule),
            Self::LoadTypes => Ok(AppEvent::LoadTypes),
            Self::ApplyFilter { page, filter } if page > 0 => {
                Ok(AppEvent::ApplyFilter { page, filter })
            }
            Self::ApplyFilter { .. } => {
                Err("Номер страницы фильтра должен быть больше нуля.".to_owned())
            }
        }
    }
}

fn default_feed_kind() -> String {
    "mine".to_owned()
}
