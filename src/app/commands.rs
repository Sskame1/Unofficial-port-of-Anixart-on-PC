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
    SearchResolved(Result<Vec<crate::features::discover::models::ReleaseCard>, String>),
    LoadDiscoverPage {
        kind: String,
        page: u32,
    },
    DiscoverPageResolved(
        Result<
            crate::features::discover::api::PageableResponse<
                crate::features::discover::models::ReleaseCard,
            >,
            String,
        >,
    ),
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
