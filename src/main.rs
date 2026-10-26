mod api;

use anyhow::{Context, Result};
use serde::Deserialize;
use tao::event::{Event, StartCause, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use tao::window::WindowBuilder;
use wry::{http::Request, WebView, WebViewBuilder};

#[derive(Debug)]
enum AppEvent {
    Play(PlayRequest),
    Resolved(Result<api::EpisodeLink, String>),
}

#[derive(Debug, Deserialize)]
struct PlayRequest {
    action: String,
    release_id: u64,
    type_id: u64,
    source_id: u64,
    position: u32,
}

fn deliver(webview: &WebView, title: &str, message: &str, url: Option<&str>) -> Result<()> {
    let payload = serde_json::json!({ "title": title, "message": message, "url": url });
    let script = format!("window.onRustMessage && window.onRustMessage({})", serde_json::to_string(&payload)?);
    webview.evaluate_script(&script).context("could not update the application window")
}

fn start() -> Result<()> {
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let proxy: EventLoopProxy<AppEvent> = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("Anixart Video Client")
        .with_inner_size(tao::dpi::LogicalSize::new(1100.0, 760.0))
        .build(&event_loop)
        .context("could not create the application window")?;

    let ipc_proxy = proxy.clone();
    let webview = WebViewBuilder::new(&window)
        .with_html(include_str!("../web/index.html"))
        .with_ipc_handler(move |request: Request<String>| {
            let parsed = serde_json::from_str::<PlayRequest>(request.body());
            match parsed {
                Ok(play) if play.action == "play" => {
                    let _ = ipc_proxy.send_event(AppEvent::Play(play));
                }
                Ok(_) => {
                    let _ = ipc_proxy.send_event(AppEvent::Resolved(Err("Unknown action.".to_owned())));
                }
                Err(error) => {
                    let _ = ipc_proxy.send_event(AppEvent::Resolved(Err(format!("Invalid request: {error}"))));
                }
            }
        })
        .build()
        .context("could not initialize WebView2")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                let _ = deliver(&webview, "Ready", "Enter Anixart episode IDs and choose Open episode.", None);
            }
            Event::UserEvent(AppEvent::Play(request)) => {
                let _ = deliver(&webview, "Connecting", "Requesting the episode from Anixart…", None);
                let proxy = proxy.clone();
                std::thread::spawn(move || {
                    let result = api::resolve_episode(
                        request.release_id,
                        request.type_id,
                        request.source_id,
                        request.position,
                    )
                    .map_err(|error| format!("{error:#}"));
                    let _ = proxy.send_event(AppEvent::Resolved(result));
                });
            }
            Event::UserEvent(AppEvent::Resolved(Ok(episode))) => {
                let _ = deliver(&webview, &episode.name, "Opening the provider player…", Some(&episode.url));
            }
            Event::UserEvent(AppEvent::Resolved(Err(error))) => {
                let _ = deliver(&webview, "Could not open episode", &error, None);
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. }
            | Event::WindowEvent { event: WindowEvent::Destroyed, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn main() {
    if let Err(error) = start() {
        eprintln!("Anixart Video Client: {error:#}");
    }
}
