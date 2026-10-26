use crate::{
    api::{client::ApiClient, error::ApiError},
    features::auth::{api, session::AuthSession},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    time::Duration,
};

// This is the Android project's public Firebase web API key. It identifies the
// Firebase project; it is not an account credential. The OAuth client ID must
// be supplied by the desktop build because the APK client is Android-only.
const FIREBASE_API_KEY: &str = "AIzaSyBFPckWOsp0MEqb_1gwszvM1ILdUixM-uw";
const DEFAULT_ANDROID_CLIENT_ID: &str =
    "983926366374-c3jvolf9t8e151mbgriuqi7uqnbt4c4p.apps.googleusercontent.com";

fn random_string(length: usize) -> String {
    let mut bytes = vec![0; length];
    // Windows CNG-backed system RNG. PKCE verifier and OAuth state must not be
    // predictable from process id or clock values.
    #[link(name = "advapi32")]
    unsafe extern "system" {
        fn SystemFunction036(buffer: *mut u8, length: u32) -> u8;
    }
    let success = unsafe { SystemFunction036(bytes.as_mut_ptr(), bytes.len() as u32) };
    if success == 0 {
        return String::new();
    }
    URL_SAFE_NO_PAD.encode(bytes)
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&value[index + 1..index + 3], 16) {
                output.push(byte);
                index += 3;
                continue;
            }
        }
        output.push(if bytes[index] == b'+' {
            b' '
        } else {
            bytes[index]
        });
        index += 1;
    }
    String::from_utf8_lossy(&output).into_owned()
}

fn callback_query(stream: &mut TcpStream) -> Result<String, ApiError> {
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .map_err(|error| ApiError::InvalidResponse(error.to_string()))?;
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 1024];
    while !bytes.windows(4).any(|window| window == b"\r\n\r\n") && bytes.len() < 16 * 1024 {
        let read = stream.read(&mut chunk).map_err(|error| {
            ApiError::InvalidResponse(format!("Google callback read failed: {error}"))
        })?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..read]);
    }
    let request = String::from_utf8_lossy(&bytes);
    request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .map(str::to_owned)
        .ok_or_else(|| ApiError::InvalidResponse("Google вернул пустой callback.".to_owned()))
}

pub fn sign_in(client: &ApiClient) -> Result<AuthSession, ApiError> {
    let client_id = std::env::var("ANIXART_GOOGLE_CLIENT_ID").unwrap_or_default();
    if client_id.is_empty() {
        return Err(ApiError::InvalidResponse(format!(
            "Google-вход требует OAuth client ID типа Desktop app. Задайте переменную ANIXART_GOOGLE_CLIENT_ID (найденный в APK Android ID: {DEFAULT_ANDROID_CLIENT_ID} для localhost не подходит)."
        )));
    }

    let listener = TcpListener::bind(("127.0.0.1", 0)).map_err(|error| {
        ApiError::InvalidResponse(format!("Не удалось открыть OAuth callback: {error}"))
    })?;
    let port = listener
        .local_addr()
        .map_err(|error| ApiError::InvalidResponse(error.to_string()))?
        .port();
    let redirect_uri = format!("http://127.0.0.1:{port}/oauth2callback");
    let state = random_string(32);
    let verifier = random_string(64);
    if state.is_empty() || verifier.is_empty() {
        return Err(ApiError::InvalidResponse(
            "Не удалось получить криптографически стойкие случайные данные Windows.".to_owned(),
        ));
    }
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}&code_challenge={}&code_challenge_method=S256",
        percent_encode(&client_id),
        percent_encode(&redirect_uri),
        percent_encode(&state),
        percent_encode(&challenge),
    );
    Command::new("cmd")
        .args(["/C", "start", "", &url])
        .spawn()
        .map_err(|error| {
            ApiError::InvalidResponse(format!("Не удалось открыть браузер Google: {error}"))
        })?;

    let (mut stream, _) = listener.accept().map_err(|error| {
        ApiError::InvalidResponse(format!("Google OAuth callback не получен: {error}"))
    })?;
    let path = callback_query(&mut stream)?;
    let _ = stream.write_all(
        b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n<p>Google sign-in complete. You can close this window.</p>",
    );
    let query = path.split('?').nth(1).unwrap_or_default();
    let mut code = None;
    let mut returned_state = None;
    let mut error = None;
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or_default();
        let value = percent_decode(parts.next().unwrap_or_default());
        match key {
            "code" => code = Some(value),
            "state" => returned_state = Some(value),
            "error" => error = Some(value),
            _ => {}
        }
    }
    if let Some(error) = error {
        return Err(ApiError::InvalidResponse(format!(
            "Google OAuth завершился ошибкой: {error}"
        )));
    }
    if returned_state.as_deref() != Some(state.as_str()) {
        return Err(ApiError::InvalidResponse(
            "Google OAuth state mismatch.".to_owned(),
        ));
    }
    let code = code.ok_or_else(|| {
        ApiError::InvalidResponse("Google OAuth не вернул authorization code.".to_owned())
    })?;

    let token_url = reqwest::Url::parse("https://oauth2.googleapis.com/token")
        .map_err(|error| ApiError::InvalidUrl(error.to_string()))?;
    let token: GoogleTokenResponse = client.post_form_absolute(
        token_url,
        &[
            ("client_id", client_id.as_str()),
            ("code", code.as_str()),
            ("code_verifier", verifier.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
        ],
    )?;
    let google_id_token = token
        .id_token
        .ok_or_else(|| ApiError::InvalidResponse("Google не вернул ID token.".to_owned()))?;

    // FirebaseAuth.signInWithCredential in the APK produces this Firebase ID
    // token. Anixart's auth/google endpoint expects that token, not Google's
    // original OAuth ID token.
    let firebase_url = reqwest::Url::parse(&format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key={FIREBASE_API_KEY}"
    ))
    .map_err(|error| ApiError::InvalidUrl(error.to_string()))?;
    let firebase: FirebaseIdentityResponse = client.post_json_absolute(
        firebase_url,
        &FirebaseIdentityRequest {
            post_body: format!(
                "id_token={}&providerId=google.com",
                percent_encode(&google_id_token)
            ),
            request_uri: redirect_uri,
            return_idp_credential: true,
            return_secure_token: true,
        },
    )?;
    let firebase_id_token = firebase
        .id_token
        .ok_or_else(|| ApiError::InvalidResponse("Firebase не вернул ID token.".to_owned()))?;
    api::sign_in_google(client, &firebase_id_token)
}

#[derive(serde::Deserialize)]
struct GoogleTokenResponse {
    id_token: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FirebaseIdentityRequest {
    post_body: String,
    request_uri: String,
    return_idp_credential: bool,
    return_secure_token: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FirebaseIdentityResponse {
    id_token: Option<String>,
}
