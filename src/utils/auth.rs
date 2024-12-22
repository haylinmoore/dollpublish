use crate::error::{AppError, Result};
use crate::models::user::Users;
use axum::http::HeaderMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn authenticate(headers: &HeaderMap, users: &Arc<Mutex<Users>>) -> Result<String> {
    let api_key = headers
        .get("api-key")
        .ok_or(AppError::AuthenticationError)?
        .to_str()
        .map_err(|_| AppError::AuthenticationError)?;

    let api_secret = headers
        .get("api-secret")
        .ok_or(AppError::AuthenticationError)?
        .to_str()
        .map_err(|_| AppError::AuthenticationError)?;

    let users = users.lock().await;
    users
        .verify_credentials(api_key, api_secret)
        .await
        .ok_or(AppError::AuthenticationError)
}
