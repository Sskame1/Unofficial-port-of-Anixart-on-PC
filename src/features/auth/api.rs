use crate::api::{client::ApiClient, error::ApiError};
use crate::features::auth::models::*;

pub fn sign_in(client: &ApiClient, request: &SignInRequest) -> Result<String, ApiError> {
    let url = client.url("auth/signIn")?;
    let response: AuthResponse = client.post_json_with_header(url, request, "", "")?;
    
    if response.code != 0 {
        return Err(ApiError::InvalidResponse(format!(
            "Anixart rejected sign-in (code {}): {}",
            response.code, response.message
        )));
    }

    if response.token.is_empty() {
        return Err(ApiError::InvalidResponse(
            "Server returned no token.".to_owned()
        ));
    }

    Ok(response.token)
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
            "Server returned no token.".to_owned()
        ));
    }

    Ok(response.token)
}
