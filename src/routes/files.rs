use crate::{error::AppError, utils::auth::authenticate, AppState};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, put},
    Router,
};
use std::fs;

const ALLOWED_FILES: [&str; 2] = ["template.html", "index.html"];

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/_files/:filename", get(get_file))
        .route("/_files/:filename", put(put_file))
}

async fn get_file(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let username = authenticate(&headers, &state.users).await?;

    if !ALLOWED_FILES.contains(&filename.as_str()) {
        return Err(AppError::NotFound);
    }

    let file_path = state.data_dir.join(&username).join(&filename);
    match fs::read(&file_path) {
        Ok(content) => Ok((StatusCode::OK, content)),
        Err(_) => Err(AppError::NotFound),
    }
}

async fn put_file(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(filename): Path<String>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let username = authenticate(&headers, &state.users).await?;

    if !ALLOWED_FILES.contains(&filename.as_str()) {
        return Err(AppError::InvalidFile);
    }

    let user_dir = state.data_dir.join(&username);
    fs::create_dir_all(&user_dir).map_err(|e| AppError::Internal(e.to_string()))?;

    let file_path = user_dir.join(&filename);
    fs::write(&file_path, body).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(StatusCode::OK)
}
