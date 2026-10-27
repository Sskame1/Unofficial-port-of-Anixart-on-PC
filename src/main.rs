mod api;
mod app;
mod features;

use anyhow::{Context, Result};
use app::{
    commands::{AppCommand, AppEvent},
    state::AppState,
};
use tao::event::{Event, StartCause, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use tao::window::{Icon, WindowBuilder};
use wry::{WebContext, WebView, WebViewBuilder, http::Request};

fn deliver(webview: &WebView, event: &str, payload: serde_json::Value) -> Result<()> {
    let message = serde_json::json!({ "event": event, "payload": payload });
    let script = format!(
        "window.app && window.app.receive({})",
        serde_json::to_string(&message)?
    );
    webview
        .evaluate_script(&script)
        .context("could not update the application window")
}

fn start() -> Result<()> {
    let api = api::ApiClient::new().context("could not initialize Anixart API client")?;
    let mut state = AppState::new(api);
    state.session = features::auth::session::load().ok().flatten();
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let proxy: EventLoopProxy<AppEvent> = event_loop.create_proxy();
    let window_icon = load_window_icon()?;
    let window = WindowBuilder::new()
        .with_title("Anixart")
        .with_window_icon(Some(window_icon))
        .with_min_inner_size(tao::dpi::LogicalSize::new(960.0, 640.0))
        .with_maximized(true)
        .build(&event_loop)
        .context("could not create the application window")?;

    let ipc_proxy = proxy.clone();
    let preferred_webview_data = std::env::var_os("LOCALAPPDATA")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("AnixartWindowsClient")
        .join("WebView2");
    let webview_data = if std::fs::create_dir_all(&preferred_webview_data).is_ok() {
        preferred_webview_data
    } else {
        let executable_dir = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(std::path::Path::to_path_buf))
            .unwrap_or_else(std::env::temp_dir);
        let fallback = executable_dir.join("AnixartWindowsClient.WebView2");
        std::fs::create_dir_all(&fallback).context(
            "could not create WebView2 profile directory in LocalAppData or next to the executable",
        )?;
        fallback
    };
    let mut web_context = WebContext::new(Some(webview_data));
    let webview = WebViewBuilder::new(&window)
        .with_web_context(&mut web_context)
        .with_html(include_str!(concat!(env!("OUT_DIR"), "/index.html")))
        .with_ipc_handler(move |request: Request<String>| {
            let result = serde_json::from_str::<AppCommand>(request.body())
                .map_err(|error| format!("Invalid request: {error}"))
                .and_then(AppCommand::into_event);
            match result {
                Ok(event) => {
                    let _ = ipc_proxy.send_event(event);
                }
                Err(message) => {
                    let _ = ipc_proxy.send_event(AppEvent::Alert(message));
                }
            }
        })
        .build()
        .context("could not initialize WebView2")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                let _ = deliver(
                    &webview,
                    "ready",
                    serde_json::json!({
                        "route": state.route(),
                        "session": state.session,
                    }),
                );
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result =
                            features::profile::api::profile(&api, session.id, &session.token)
                                .map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::ProfileResolved(result));
                    });
                }
            }
            Event::UserEvent(AppEvent::Navigate(route)) => {
                if state.navigate(route.clone()).is_ok() {
                    let _ = deliver(&webview, "navigate", serde_json::json!({ "route": route }));
                }
            }
            Event::UserEvent(AppEvent::PlayEpisode {
                release_id,
                type_id,
                source_id,
                position,
            }) => {
                let _ = deliver(
                    &webview,
                    "status",
                    serde_json::json!({
                        "title": "Connecting",
                        "message": "Requesting the episode from Anixart…"
                    }),
                );
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::player::resolver::resolve_episode(
                        &api, release_id, type_id, source_id, position,
                    )
                    .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::EpisodeResolved(result));
                });
            }
            Event::UserEvent(AppEvent::DiscoverRandomRelease) => {
                let _ = deliver(
                    &webview,
                    "status",
                    serde_json::json!({
                        "title": "Обзор",
                        "message": "Загружаем случайный релиз из гостевого каталога…"
                    }),
                );
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::discover::api::random_release(&api)
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::RandomReleaseResolved(result));
                });
            }
            Event::UserEvent(AppEvent::OpenRelease(release_id)) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                let token = state.session.as_ref().map(|session| session.token.clone());
                std::thread::spawn(move || {
                    let result = features::discover::api::release_with_token(&api, release_id, token.as_deref())
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::ReleaseResolved(result));
                });
            }
            Event::UserEvent(AppEvent::LoadEpisodeTypes(release_id)) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::episodes::api::types(&api, release_id)
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::EpisodeTypesResolved(result));
                });
            }
            Event::UserEvent(AppEvent::LoadEpisodeSources {
                release_id,
                type_id,
            }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::episodes::api::sources(&api, release_id, type_id)
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::EpisodeSourcesResolved(result));
                });
            }
            Event::UserEvent(AppEvent::LoadEpisodes {
                release_id,
                type_id,
                source_id,
            }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result =
                        features::episodes::api::episodes(&api, release_id, type_id, source_id)
                            .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::EpisodesResolved(result));
                });
            }
            Event::UserEvent(AppEvent::SearchReleases(query)) => {
                let _ = deliver(
                    &webview,
                    "search_pending",
                    serde_json::json!({ "query": query }),
                );
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::discover::search::search_releases(&api, &query)
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::SearchResolved(result));
                });
            }
            Event::UserEvent(AppEvent::SignIn { login, password }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let request = features::auth::models::SignInRequest { login, password };
                    let result = features::auth::api::sign_in(&api, &request)
                        .and_then(|session| {
                            features::auth::session::save(&session)
                                .map_err(|error| {
                                    crate::api::error::ApiError::InvalidResponse(error)
                                })
                                .map(|_| session)
                        })
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::AuthResolved(result));
                });
            }
            Event::UserEvent(AppEvent::SignInGoogle) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::auth::google::sign_in(&api)
                        .and_then(|session| {
                            features::auth::session::save(&session)
                                .map_err(|error| {
                                    crate::api::error::ApiError::InvalidResponse(error)
                                })
                                .map(|_| session)
                        })
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::AuthResolved(result));
                });
            }
            Event::UserEvent(AppEvent::SignOut) => {
                state.session = None;
                let result = features::auth::session::clear();
                let _ = deliver(
                    &webview,
                    "signed_out",
                    serde_json::json!({ "ok": result.is_ok(), "message": result.err() }),
                );
            }
            Event::UserEvent(AppEvent::LoadBookmarks { kind, page }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result =
                            features::account::api::bookmarks(&api, &session.token, &kind, page)
                                .map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::BookmarksResolved(result));
                    });
                } else {
                    let _ = deliver(
                        &webview,
                        "account_error",
                        serde_json::json!({"message": "Войдите в аккаунт для просмотра закладок."}),
                    );
                }
            }
            Event::UserEvent(AppEvent::LoadFeed { page, request_id }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::account::api::feed(&api, &session.token, page)
                            .map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::FeedResolved { request_id, result });
                    });
                } else {
                    let _ = deliver(
                        &webview,
                        "account_error",
                        serde_json::json!({"message": "Войдите в аккаунт для просмотра ленты."}),
                    );
                }
            }
            Event::UserEvent(AppEvent::LoadCollections {
                kind,
                page,
                request_id,
            }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                let token = state.session.as_ref().map(|session| session.token.clone());
                let profile_id = state.session.as_ref().map(|session| session.id);
                std::thread::spawn(move || {
                    let result = features::account::api::collections(
                        &api,
                        token.as_deref(),
                        &kind,
                        page,
                        profile_id,
                    )
                    .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::CollectionsResolved { request_id, result });
                });
            }
            Event::UserEvent(AppEvent::OpenCollection {
                collection_id,
                request_id,
            }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                let token = state.session.as_ref().map(|session| session.token.clone());
                std::thread::spawn(move || {
                    let result =
                        features::account::api::collection(&api, token.as_deref(), collection_id)
                            .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::CollectionResolved { request_id, result });
                });
            }
            Event::UserEvent(AppEvent::ToggleCollectionFavorite { collection_id, enabled }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::account::api::toggle_collection_favorite(&api, &session.token, collection_id, enabled)
                            .map(|_| enabled).map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::CollectionFavoriteResolved(result));
                    });
                } else {
                    let _ = deliver(&webview, "collection_favorite_error", serde_json::json!({"message": "Войдите, чтобы добавить коллекцию в закладки."}));
                }
            }
            Event::UserEvent(AppEvent::AddReleaseToList { release_id, list_type, current_status }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone(); let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::account::api::update_release_list(&api, &session.token, release_id, current_status, list_type).map_err(|e| e.to_string());
                        let _ = proxy.send_event(AppEvent::ReleaseListResolved(result));
                    });
                } else {
                    let _ = deliver(&webview, "release_list_error", serde_json::json!({"message":"Войдите в аккаунт, чтобы сохранить релиз."}));
                }
            }
            Event::UserEvent(AppEvent::ToggleReleaseFavorite { release_id, enabled }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::account::api::toggle_release_favorite(
                            &api, &session.token, release_id, enabled,
                        ).map(|_| enabled).map_err(|e| e.to_string());
                        let _ = proxy.send_event(AppEvent::ReleaseFavoriteResolved(result));
                    });
                } else {
                    let _ = deliver(&webview, "release_favorite_error", serde_json::json!({"message":"Войдите в аккаунт, чтобы изменить избранное."}));
                }
            }
            Event::UserEvent(AppEvent::VoteRelease { release_id, vote }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::account::api::vote_release(
                            &api, &session.token, release_id, vote,
                        ).map(|_| vote).map_err(|e| e.to_string());
                        let _ = proxy.send_event(AppEvent::ReleaseVoteResolved(result));
                    });
                } else {
                    let _ = deliver(&webview, "release_vote_error", serde_json::json!({"message":"Войдите в аккаунт, чтобы оценить релиз."}));
                }
            }
            Event::UserEvent(AppEvent::LoadFriends { page }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result =
                            features::profile::api::friends(&api, session.id, page, &session.token)
                                .map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::FriendsResolved(result));
                    });
                } else {
                    let _ = deliver(
                        &webview,
                        "friends_error",
                        serde_json::json!({"message": "Войдите в аккаунт для просмотра друзей."}),
                    );
                }
            }
            Event::UserEvent(AppEvent::RefreshProfile) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone(); let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::profile::api::profile(&api, session.id, &session.token).map_err(|e| e.to_string());
                        let _ = proxy.send_event(AppEvent::ProfileResolved(result));
                    });
                }
            }
            Event::UserEvent(AppEvent::LoadReleaseComments { release_id, page, sort }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                let token = state.session.as_ref().map(|session| session.token.clone());
                std::thread::spawn(move || {
                    let result = features::reviews::release_comments(
                        &api,
                        release_id,
                        page,
                        sort,
                        token.as_deref(),
                    )
                    .map(|page| serde_json::to_value(page).unwrap_or_default())
                    .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::ReleaseCommentsResolved(result));
                });
            }
            Event::UserEvent(AppEvent::VoteReleaseComment { comment_id, vote }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::reviews::vote_release_comment(
                            &api,
                            comment_id,
                            vote,
                            &session.token,
                        )
                        .map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::ReleaseCommentVoteResolved(result));
                    });
                } else {
                    let _ = deliver(
                        &webview,
                        "release_comment_vote_error",
                        serde_json::json!({"message": "Войдите в аккаунт, чтобы оценивать отзывы."}),
                    );
                }
            }
            Event::UserEvent(AppEvent::LoadArticle(article_id)) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                let token = state.session.as_ref().map(|session| session.token.clone());
                std::thread::spawn(move || {
                    let result =
                        features::account::api::article(&api, token.as_deref(), article_id)
                            .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::ArticleResolved(result));
                });
            }
            Event::UserEvent(AppEvent::LoadHistory { page }) => {
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result = features::account::api::history(&api, &session.token, page)
                            .map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::HistoryResolved(result));
                    });
                } else {
                    let _ = deliver(
                        &webview,
                        "account_error",
                        serde_json::json!({"message": "Войдите в аккаунт для просмотра истории."}),
                    );
                }
            }
            Event::UserEvent(AppEvent::LoadDiscoverPage { kind, page }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                let token = state.session.as_ref().map(|session| session.token.clone());
                std::thread::spawn(move || {
                    let kind = match kind.as_str() {
                        "recommendations" => features::discover::api::DiscoverKind::Recommendations,
                        "watching" => features::discover::api::DiscoverKind::Watching,
                        "interesting" => features::discover::api::DiscoverKind::Interesting,
                        _ => features::discover::api::DiscoverKind::Discussing,
                    };
                    let result = features::discover::api::page(&api, kind, page, token.as_deref())
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::DiscoverPageResolved(result));
                });
            }
            Event::UserEvent(AppEvent::LoadSchedule) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result =
                        features::schedule::api::week(&api).map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::ScheduleResolved(result));
                });
            }
            Event::UserEvent(AppEvent::LoadTypes) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::types::api::all(&api).map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::TypesResolved(result));
                });
            }
            Event::UserEvent(AppEvent::ApplyFilter { page, filter }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = features::filter::api::search(&api, page, &filter)
                        .map_err(|error| error.to_string());
                    let _ = proxy.send_event(AppEvent::FilterResolved(result));
                });
            }
            Event::UserEvent(AppEvent::SearchResolved(Ok(releases))) => {
                let _ = deliver(
                    &webview,
                    "search_results",
                    serde_json::to_value(releases).unwrap_or_else(|_| serde_json::json!([])),
                );
            }
            Event::UserEvent(AppEvent::SearchResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "search_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::AuthResolved(Ok(token))) => {
                state.session = Some(token.clone());
                let _ = deliver(
                    &webview,
                    "auth_success",
                    serde_json::to_value(token).unwrap_or_else(|_| serde_json::json!({})),
                );
                if let Some(session) = state.session.clone() {
                    let api = state.api.clone();
                    let proxy = proxy.clone();
                    std::thread::spawn(move || {
                        let result =
                            features::profile::api::profile(&api, session.id, &session.token)
                                .map_err(|error| error.to_string());
                        let _ = proxy.send_event(AppEvent::ProfileResolved(result));
                    });
                }
            }
            Event::UserEvent(AppEvent::AuthResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "auth_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::ProfileResolved(Ok(profile))) => {
                let _ = deliver(
                    &webview,
                    "profile_data",
                    serde_json::to_value(profile).unwrap_or_default(),
                );
            }
            Event::UserEvent(AppEvent::ProfileResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "profile_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::BookmarksResolved(Ok(page))) => {
                let _ = deliver(
                    &webview,
                    "bookmarks_data",
                    serde_json::to_value(page).unwrap_or_default(),
                );
            }
            Event::UserEvent(AppEvent::BookmarksResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "bookmarks_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::FeedResolved {
                request_id,
                result: Ok(page),
            }) => {
                let _ = deliver(
                    &webview,
                    "feed_data",
                    serde_json::json!({"request_id": request_id, "data": page}),
                );
            }
            Event::UserEvent(AppEvent::FeedResolved {
                request_id,
                result: Err(error),
            }) => {
                let _ = deliver(
                    &webview,
                    "feed_error",
                    serde_json::json!({ "request_id": request_id, "message": error }),
                );
            }
            Event::UserEvent(AppEvent::CollectionsResolved {
                request_id,
                result: Ok(page),
            }) => {
                let _ = deliver(
                    &webview,
                    "collections_data",
                    serde_json::json!({"request_id": request_id, "data": page}),
                );
            }
            Event::UserEvent(AppEvent::CollectionsResolved {
                request_id,
                result: Err(error),
            }) => {
                let _ = deliver(
                    &webview,
                    "collections_error",
                    serde_json::json!({ "request_id": request_id, "message": error }),
                );
            }
            Event::UserEvent(AppEvent::CollectionResolved {
                request_id,
                result: Ok(collection),
            }) => {
                let _ = deliver(
                    &webview,
                    "collection_data",
                    serde_json::json!({"request_id": request_id, "data": collection}),
                );
            }
            Event::UserEvent(AppEvent::CollectionResolved {
                request_id,
                result: Err(error),
            }) => {
                let _ = deliver(
                    &webview,
                    "collections_error",
                    serde_json::json!({ "request_id": request_id, "message": error }),
                );
            }
            Event::UserEvent(AppEvent::CollectionFavoriteResolved(Ok(enabled))) => {
                let _ = deliver(&webview, "collection_favorite_done", serde_json::json!({"enabled": enabled}));
            }
            Event::UserEvent(AppEvent::CollectionFavoriteResolved(Err(error))) => {
                let _ = deliver(&webview, "collection_favorite_error", serde_json::json!({"message": error}));
            }
            Event::UserEvent(AppEvent::ReleaseListResolved(Ok(()))) => { let _ = deliver(&webview, "release_list_done", serde_json::json!({})); }
            Event::UserEvent(AppEvent::ReleaseListResolved(Err(error))) => { let _ = deliver(&webview, "release_list_error", serde_json::json!({"message": error})); }
            Event::UserEvent(AppEvent::ReleaseFavoriteResolved(Ok(enabled))) => { let _ = deliver(&webview, "release_favorite_done", serde_json::json!({"enabled": enabled})); }
            Event::UserEvent(AppEvent::ReleaseFavoriteResolved(Err(error))) => { let _ = deliver(&webview, "release_favorite_error", serde_json::json!({"message": error})); }
            Event::UserEvent(AppEvent::ReleaseVoteResolved(Ok(vote))) => { let _ = deliver(&webview, "release_vote_done", serde_json::json!({"vote": vote})); }
            Event::UserEvent(AppEvent::ReleaseVoteResolved(Err(error))) => { let _ = deliver(&webview, "release_vote_error", serde_json::json!({"message": error})); }
            Event::UserEvent(AppEvent::FriendsResolved(Ok(page))) => {
                let _ = deliver(
                    &webview,
                    "friends_data",
                    serde_json::to_value(page).unwrap_or_default(),
                );
            }
            Event::UserEvent(AppEvent::FriendsResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "friends_error",
                    serde_json::json!({"message": error}),
                );
            }
            Event::UserEvent(AppEvent::ReleaseCommentsResolved(Ok(page))) => {
                let _ = deliver(&webview, "release_comments_data", page);
            }
            Event::UserEvent(AppEvent::ReleaseCommentsResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "release_comments_error",
                    serde_json::json!({"message": error}),
                );
            }
            Event::UserEvent(AppEvent::ReleaseCommentVoteResolved(Ok(()))) => {
                let _ = deliver(&webview, "release_comment_vote_done", serde_json::json!({}));
            }
            Event::UserEvent(AppEvent::ReleaseCommentVoteResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "release_comment_vote_error",
                    serde_json::json!({"message": error}),
                );
            }
            Event::UserEvent(AppEvent::ArticleResolved(Ok(article))) => {
                let _ = deliver(&webview, "article_data", article);
            }
            Event::UserEvent(AppEvent::ArticleResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "article_error",
                    serde_json::json!({"message": error}),
                );
            }
            Event::UserEvent(AppEvent::DiscoverPageResolved(Ok(page))) => {
                let _ = deliver(
                    &webview,
                    "discover_page",
                    serde_json::to_value(page).unwrap_or_else(|_| serde_json::json!({})),
                );
            }
            Event::UserEvent(AppEvent::HistoryResolved(Ok(page))) => {
                let _ = deliver(
                    &webview,
                    "history_data",
                    serde_json::to_value(page).unwrap_or_default(),
                );
            }
            Event::UserEvent(AppEvent::HistoryResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "history_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::DiscoverPageResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "discover_page_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::ScheduleResolved(Ok(schedule))) => {
                let _ = deliver(
                    &webview,
                    "schedule_data",
                    serde_json::to_value(schedule).unwrap_or_else(|_| serde_json::json!({})),
                );
            }
            Event::UserEvent(AppEvent::ScheduleResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "schedule_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::TypesResolved(Ok(types))) => {
                let _ = deliver(
                    &webview,
                    "types_data",
                    serde_json::to_value(types).unwrap_or_else(|_| serde_json::json!([])),
                );
            }
            Event::UserEvent(AppEvent::TypesResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "types_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::FilterResolved(Ok(page))) => {
                let _ = deliver(
                    &webview,
                    "filter_results",
                    serde_json::to_value(page).unwrap_or_else(|_| serde_json::json!({})),
                );
            }
            Event::UserEvent(AppEvent::FilterResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "filter_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::RandomReleaseResolved(Ok(release))) => {
                let _ = deliver(
                    &webview,
                    "random_release",
                    serde_json::to_value(release).unwrap_or_else(|_| serde_json::json!({})),
                );
            }
            Event::UserEvent(AppEvent::RandomReleaseResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "random_release_error",
                    serde_json::json!({
                        "title": "Не удалось загрузить каталог",
                        "message": error
                    }),
                );
            }
            Event::UserEvent(AppEvent::ReleaseResolved(Ok(release))) => {
                let _ = deliver(
                    &webview,
                    "release_detail",
                    serde_json::to_value(release).unwrap_or_else(|_| serde_json::json!({})),
                );
            }
            Event::UserEvent(AppEvent::ReleaseResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "feature_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::EpisodeTypesResolved(Ok(types))) => {
                let _ = deliver(
                    &webview,
                    "episode_types",
                    serde_json::to_value(types).unwrap_or_else(|_| serde_json::json!([])),
                );
            }
            Event::UserEvent(AppEvent::EpisodeTypesResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "feature_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::EpisodeSourcesResolved(Ok(sources))) => {
                let _ = deliver(
                    &webview,
                    "episode_sources",
                    serde_json::to_value(sources).unwrap_or_else(|_| serde_json::json!([])),
                );
            }
            Event::UserEvent(AppEvent::EpisodeSourcesResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "feature_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::EpisodesResolved(Ok(episodes))) => {
                let _ = deliver(
                    &webview,
                    "episodes",
                    serde_json::to_value(episodes).unwrap_or_else(|_| serde_json::json!([])),
                );
            }
            Event::UserEvent(AppEvent::EpisodesResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "feature_error",
                    serde_json::json!({ "message": error }),
                );
            }
            Event::UserEvent(AppEvent::EpisodeResolved(Ok(episode))) => {
                let _ = deliver(
                    &webview,
                    "episode",
                    serde_json::json!({
                        "name": episode.name,
                        "url": episode.url
                    }),
                );
            }
            Event::UserEvent(AppEvent::EpisodeResolved(Err(error))) => {
                let _ = deliver(
                    &webview,
                    "episode_error",
                    serde_json::json!({
                        "title": "Could not open episode",
                        "message": error
                    }),
                );
            }
            Event::UserEvent(AppEvent::Alert(message)) => {
                let _ = deliver(&webview, "alert", serde_json::json!({ "message": message }));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::Destroyed,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn load_window_icon() -> Result<Icon> {
    let decoder = png::Decoder::new(std::io::Cursor::new(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/app-icon.png"
    ))));
    let mut reader = decoder
        .read_info()
        .context("could not read application logo")?;
    let mut buffer = vec![
        0;
        reader
            .output_buffer_size()
            .context("invalid application logo dimensions")?
    ];
    let info = reader
        .next_frame(&mut buffer)
        .context("could not decode application logo")?;
    let pixels = &buffer[..info.buffer_size()];
    let rgba = match info.color_type {
        png::ColorType::Rgba => pixels.to_vec(),
        png::ColorType::Rgb => pixels
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => pixels
            .chunks_exact(2)
            .flat_map(|p| [p[0], p[0], p[0], p[1]])
            .collect(),
        png::ColorType::Grayscale => pixels.iter().flat_map(|p| [*p, *p, *p, 255]).collect(),
        png::ColorType::Indexed => {
            anyhow::bail!("application logo uses unsupported indexed PNG color")
        }
    };
    Icon::from_rgba(rgba, info.width, info.height)
        .context("could not create application window icon")
}

fn main() {
    if let Err(error) = start() {
        eprintln!("Anixart: {error:#}");
    }
}
