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
use tao::window::WindowBuilder;
use wry::{WebView, WebViewBuilder, http::Request};

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
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let proxy: EventLoopProxy<AppEvent> = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("Anixart")
        .with_inner_size(tao::dpi::LogicalSize::new(430.0, 860.0))
        .build(&event_loop)
        .context("could not create the application window")?;

    let ipc_proxy = proxy.clone();
    let webview = WebViewBuilder::new(&window)
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
                    serde_json::json!({ "route": state.route() }),
                );
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
                std::thread::spawn(move || {
                    let result = features::discover::api::release(&api, release_id)
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
            Event::UserEvent(AppEvent::LoadDiscoverPage { kind, page }) => {
                let api = state.api.clone();
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let kind = match kind.as_str() {
                        "recommendations" => features::discover::api::DiscoverKind::Recommendations,
                        "watching" => features::discover::api::DiscoverKind::Watching,
                        "interesting" => features::discover::api::DiscoverKind::Interesting,
                        _ => features::discover::api::DiscoverKind::Discussing,
                    };
                    let result = features::discover::api::page(&api, kind, page)
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
            Event::UserEvent(AppEvent::DiscoverPageResolved(Ok(page))) => {
                let _ = deliver(
                    &webview,
                    "discover_page",
                    serde_json::to_value(page).unwrap_or_else(|_| serde_json::json!({})),
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
                    "status",
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

fn main() {
    if let Err(error) = start() {
        eprintln!("Anixart: {error:#}");
    }
}
