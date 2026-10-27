use crate::api::ApiClient;
use crate::features::auth::session::AuthSession;

pub struct AppState {
    pub api: ApiClient,
    current_route: String,
    pub session: Option<AuthSession>,
}

impl AppState {
    pub fn new(api: ApiClient) -> Self {
        Self {
            api,
            current_route: "home".to_owned(),
            session: None,
        }
    }

    pub fn route(&self) -> &str {
        &self.current_route
    }

    pub fn navigate(&mut self, route: String) -> Result<(), String> {
        if !Self::is_known_route(&route) {
            return Err(format!("Unknown section: {route}"));
        }
        self.current_route = route;
        Ok(())
    }

    pub fn is_known_route(route: &str) -> bool {
        matches!(
            route,
            "home"
                | "discover"
                | "search"
                | "bookmarks"
                | "feed"
                | "profile"
                | "release"
                | "watch"
                | "reviews"
                | "schedule"
                | "collections"
                | "filter"
                | "signin"
                | "signup"
                | "restore"
                | "verify"
                | "settings"
                | "notifications"
        )
    }
}
