use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlaybackProgress {
    pub position: u64,
    pub watched: bool,
}

fn storage_path() -> PathBuf {
    let root = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("AnixartWindowsClient");
    root.join("playback.json")
}

pub fn load() -> HashMap<String, PlaybackProgress> {
    let path = storage_path();
    let Ok(bytes) = fs::read(path) else {
        return HashMap::new();
    };
    serde_json::from_slice(&bytes).unwrap_or_default()
}

pub fn save(progress: &HashMap<String, PlaybackProgress>) -> Result<(), String> {
    let path = storage_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let bytes = serde_json::to_vec_pretty(progress).map_err(|error| error.to_string())?;
    let temp = path.with_extension("json.tmp");
    fs::write(&temp, bytes).map_err(|error| error.to_string())?;
    fs::rename(&temp, path).map_err(|error| error.to_string())
}
