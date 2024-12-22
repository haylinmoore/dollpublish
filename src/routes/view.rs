use crate::{error::Result, models::post::Post};
use axum::{
    extract::{Path, State},
    response::Html,
};

pub async fn view_post(
    State(state): State<crate::AppState>,
    Path((username, id)): Path<(String, String)>,
) -> Result<Html<String>> {
    let post = Post::load_by_path(&state.data_dir, &username, &id).await?;
    let rendered_content = post.render_content();
    let html = state
        .templates
        .render(&state.data_dir, &username, &post, &rendered_content);

    Ok(Html(html))
}

pub fn view_routes() -> axum::Router<crate::AppState> {
    axum::Router::new().route("/:username/:id", axum::routing::get(view_post))
}
