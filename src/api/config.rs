use std::time::Duration;

pub const API_SEED: &str = "https://api-s.anixsekai.com/";
pub const VERSION_CODE: u32 = 26_101_718;
// Match the UA used by AniDesk's working AnixartJS client. Some API edges
// behave differently for clients that do not identify as the Android app.
pub const USER_AGENT: &str =
    "AnixartApp/9.0 BETA 7-25082901 (Android 9; SDK 28; x86_64; ROG ASUS AI2201_B; ru)";
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
