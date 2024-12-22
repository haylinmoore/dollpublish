use crate::error::{AppError, Result};
use crate::models::user::Users;
use axum::http::HeaderMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn authenticate(headers: &HeaderMap, users: &Arc<Mutex<Users>>) -> Result<String> {
    let api_key = headers
        .get("api-key")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let api_secret = headers
        .get("api-secret")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if api_key.is_empty() && api_secret.is_empty() {
        return Err(AppError::AuthenticationError);
    }

    let mut users = users.lock().await;
    users
        .verify_credentials(api_key, api_secret)
        .await
        .ok_or(AppError::AuthenticationError)
}
