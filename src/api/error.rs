use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    Transport(reqwest::Error),
    HttpStatus(reqwest::StatusCode),
    InvalidJson(serde_json::Error),
    InvalidUrl(String),
    InvalidResponse(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) if error.is_timeout() => write!(f, "request timed out: {error}"),
            Self::Transport(error) => write!(f, "request failed: {error}"),
            Self::HttpStatus(status) => write!(f, "API returned HTTP status {status}"),
            Self::InvalidJson(error) => write!(f, "API returned invalid JSON: {error}"),
            Self::InvalidUrl(error) => write!(f, "API returned an invalid URL: {error}"),
            Self::InvalidResponse(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::InvalidJson(error) => Some(error),
            Self::HttpStatus(_) | Self::InvalidUrl(_) | Self::InvalidResponse(_) => None,
        }
    }
}
