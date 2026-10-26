use keyring::Entry;
use serde::{Deserialize, Serialize};

const SERVICE: &str = "anixart-windows-client";
const ACCOUNT: &str = "profile-session";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub id: u64,
    pub token: String,
}

fn entry() -> Result<Entry, String> {
    Entry::new(SERVICE, ACCOUNT).map_err(|error| format!("Credential Manager: {error}"))
}

pub fn save(session: &AuthSession) -> Result<(), String> {
    let value = serde_json::to_string(session).map_err(|error| error.to_string())?;
    entry()?
        .set_password(&value)
        .map_err(|error| format!("Credential Manager: {error}"))
}

pub fn load() -> Result<Option<AuthSession>, String> {
    let value = match entry()?.get_password() {
        Ok(value) => value,
        Err(keyring::Error::NoEntry) => return Ok(None),
        Err(error) => return Err(format!("Credential Manager: {error}")),
    };
    let session = serde_json::from_str(&value)
        .map_err(|error| format!("Saved session is invalid: {error}"))?;
    Ok(Some(session))
}

pub fn clear() -> Result<(), String> {
    match entry()?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(error) => Err(format!("Credential Manager: {error}")),
    }
}
