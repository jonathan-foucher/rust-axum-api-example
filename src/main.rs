use axum::{routing::get, Router};
use std::env;

const DEFAULT_HTTP_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let http_port: u16 = env::var("HTTP_PORT")
        .map(|http_port| http_port.parse::<u16>().unwrap_or(DEFAULT_HTTP_PORT))
        .unwrap_or(DEFAULT_HTTP_PORT);

    let app = Router::new()
        .route("/", get(root));

    log::info!("Application is starting on port {}", http_port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", http_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
