mod error;
mod models;
mod routes;
mod utils;

use axum::Router;
use models::user::Users;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use utils::template::Templates;

#[derive(Clone)]
pub struct AppState {
    users: Arc<Mutex<Users>>,
    data_dir: PathBuf,
    templates: Templates,
}

#[tokio::main]
async fn main() {
    let data_dir = PathBuf::from("./data");
    let users = Users::load_or_create(&data_dir)
        .await
        .expect("Failed to initialize users");

    let templates = Templates::new();

    let state = AppState {
        users,
        data_dir,
        templates,
    };

    let app = Router::new()
        .merge(routes::moon::routes::moon_routes())
        .merge(routes::view::view_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
