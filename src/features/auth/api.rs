#![allow(dead_code)]

use crate::api::{client::ApiClient, error::ApiError};
use crate::features::auth::{models::*, session::AuthSession};
use std::time::Duration;

pub fn sign_in(client: &ApiClient, request: &SignInRequest) -> Result<AuthSession, ApiError> {
    let url = client.url("auth/signIn")?;
    let response: AuthResponse = client.post_form_with_timeout(
        url,
        &[
            ("login", request.login.as_str()),
            ("password", request.password.as_str()),
        ],
        None,
        Duration::from_secs(25),
    )?;

    if response.code != 0 {
        let reason = match response.code {
            2 => "Логин или email не найден.",
            3 => "Неверный пароль.",
            402 => "Аккаунт временно заблокирован.",
            403 => "Аккаунт заблокирован навсегда.",
            _ if response.message.is_empty() => "Проверьте данные и попробуйте ещё раз.",
            _ => response.message.as_str(),
        };
        return Err(ApiError::InvalidResponse(format!(
            "{} (код {})",
            reason, response.code
        )));
    }

    let token = response
        .profile_token
        .map(|value| value.token)
        .unwrap_or_default();
    if token.is_empty() {
        return Err(ApiError::InvalidResponse(
            "Вход отклонён: сервер не вернул токен сессии.".to_owned(),
        ));
    }

    let id = response.profile.map(|profile| profile.id).ok_or_else(|| {
        ApiError::InvalidResponse("Вход выполнен, но сервер не вернул ID профиля.".to_owned())
    })?;
    Ok(AuthSession { id, token })
}

pub fn sign_in_google(client: &ApiClient, id_token: &str) -> Result<AuthSession, ApiError> {
    let url = client.url("auth/google")?;
    let response: AuthResponse = client.post_form(url, &[("googleIdToken", id_token)], None)?;
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected Google sign-in (code {}): {}",
            response.code, response.message
        )));
    }
    let token = response
        .profile_token
        .map(|value| value.token)
        .unwrap_or(response.token);
    if token.is_empty() {
        return Err(ApiError::InvalidResponse(
            "Google sign-in returned no session token.".to_owned(),
        ));
    }
    let id = response.profile.map(|profile| profile.id).ok_or_else(|| {
        ApiError::InvalidResponse(
            "Google-вход выполнен, но сервер не вернул ID профиля.".to_owned(),
        )
    })?;
    Ok(AuthSession { id, token })
}

pub fn sign_up(client: &ApiClient, request: &SignUpRequest) -> Result<String, ApiError> {
    let url = client.url("auth/signUp")?;
    let response: AuthResponse = client.post_json_with_header(url, request, "", "")?;

    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected sign-up (code {}): {}",
            response.code, response.message
        )));
    }

    Ok(response.message)
}

pub fn restore(client: &ApiClient, request: &RestoreRequest) -> Result<String, ApiError> {
    let url = client.url("auth/restore")?;
    let response: AuthResponse = client.post_json_with_header(url, request, "", "")?;

    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected restore (code {}): {}",
            response.code, response.message
        )));
    }

    Ok(response.message)
}

pub fn verify(client: &ApiClient, request: &VerifyRequest) -> Result<String, ApiError> {
    let url = client.url("auth/verify")?;
    let response: AuthResponse = client.post_json_with_header(url, request, "", "")?;

    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected verification (code {}): {}",
            response.code, response.message
        )));
    }

    if response.token.is_empty() {
        return Err(ApiError::InvalidResponse(
            "Server returned no token.".to_owned(),
        ));
    }

    Ok(response.token)
}
