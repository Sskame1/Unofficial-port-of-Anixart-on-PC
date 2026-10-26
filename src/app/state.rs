use crate::api::ApiClient;

pub struct AppState {
    pub api: ApiClient,
    current_route: String,
}

impl AppState {
    pub fn new(api: ApiClient) -> Self {
        Self {
            api,
            current_route: "home".to_owned(),
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
