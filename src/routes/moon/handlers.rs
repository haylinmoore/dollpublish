use crate::{
    error::Result,
    models::{metadata::Metadata, post::Post},
    utils::auth::authenticate,
};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use std::collections::HashMap;

pub async fn publish(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
    Json(mut data): Json<Post>,
) -> Result<Json<Metadata>> {
    let username = authenticate(&headers, &state.users).await?;

    let id = match data.metadata.id {
        Some(ref id) => id.clone(),
        None => crate::utils::id_generator::generate_id(),
    };

    data.metadata.id = Some(id.clone());
    data.save(&state.data_dir, &username, &id).await?;

    Ok(Json(data.metadata))
}

pub async fn republish(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(mut data): Json<Post>,
) -> Result<Json<Metadata>> {
    let username = authenticate(&headers, &state.users).await?;

    data.metadata.id = Some(id.clone());
    data.save(&state.data_dir, &username, &id).await?;

    Ok(Json(data.metadata))
}

pub async fn unpublish(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Metadata>> {
    let username = authenticate(&headers, &state.users).await?;
    Post::delete(&state.data_dir, &username, &id).await?;

    Ok(Json(Metadata {
        id: None,
        extra: HashMap::new(),
    }))
}

pub async fn detail(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Post>> {
    let username = authenticate(&headers, &state.users).await?;
    let data = Post::load(&state.data_dir, &username, &id).await?;
    Ok(Json(data))
}
