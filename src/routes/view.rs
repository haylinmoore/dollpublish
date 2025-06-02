use crate::{
    error::{AppError, Result},
    models::post::Post,
};
use axum::body::Body;
use axum::http::{header, HeaderMap};
use axum::response::{Redirect, Response};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn view_post(
    State(state): State<crate::AppState>,
    Path((username, id)): Path<(String, String)>,
) -> Result<Html<String>> {
    let post = Post::load(&state.data_dir, &username, &id).await?;
    let rendered_content = post.render_content();
    let html = state
        .templates
        .render(&state.data_dir, &username, &post, &rendered_content);

    Ok(Html(html))
}

pub async fn serve_attachment(
    State(state): State<crate::AppState>,
    Path((username, id, filename)): Path<(String, String, String)>,
) -> Result<Response<Body>> {
    // Construct the file path
    let mut path = PathBuf::from(&state.data_dir);
    path.push(&username);
    path.push(&id);
    path.push("attachments");
    path.push(&filename);

    let file = match File::open(&path).await {
        Ok(file) => file,
        Err(_) => return Err(AppError::NotFound),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    if let Some(mime_type) = mime_guess::from_path(&path).first_raw() {
        headers.insert(header::CONTENT_TYPE, mime_type.parse().unwrap());
    }

    Ok((headers, body).into_response())
}

pub async fn redirect_to_github() -> Redirect {
    Redirect::permanent("https://github.com/haylinmoore/dollpublish")
}

pub async fn view_user_index(
    State(state): State<crate::AppState>,
    Path(username): Path<String>,
) -> Result<Html<String>> {
    match Post::load(&state.data_dir, &username, "index").await {
        Ok(post) => {
            let rendered_content = post.render_content();
            let html = state
                .templates
                .render(&state.data_dir, &username, &post, &rendered_content);
            Ok(Html(html))
        }
        Err(_) => {
            let message = format!(
                "{} does not have a landing page, if you are {} you can create one by making a post with the id index",
                username, username
            );
            Ok(Html(format!("<html><body><p>{}</p></body></html>", message)))
        }
    }
}

pub fn view_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(redirect_to_github))
        .route("/:username", axum::routing::get(view_user_index))
        .route("/:username/", axum::routing::get(view_user_index))
        .route("/:username/:id/", axum::routing::get(view_post))
        .route(
            "/:username/:id/attachments/:file",
            axum::routing::get(serve_attachment),
        )
}
