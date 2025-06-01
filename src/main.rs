mod error;
mod models;
mod routes;
mod utils;

use axum::Router;
use dotenvy::dotenv;
use models::user::Users;
use std::{env, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use utils::template::Templates;

#[derive(Clone)]
pub struct AppState {
    users: Arc<Mutex<Users>>,
    data_dir: PathBuf,
    templates: Templates,
}

fn get_config() -> (PathBuf, String, u16) {
    dotenv().ok();

    let data_dir = env::var("MOON_DATA_DIR").unwrap_or_else(|_| "./data".to_string());
    let bind_addr = env::var("MOON_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("MOON_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    (PathBuf::from(data_dir), bind_addr, port)
}

#[tokio::main]
async fn main() {
    // Get configuration
    let (data_dir, bind_addr, port) = get_config();

    // Create data directory if it doesn't exist
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
    }

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
        .merge(routes::files::file_routes())
        .with_state(state);

    let addr = format!("{}:{}", bind_addr, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {}", addr));

    println!("dollpublish has started on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
