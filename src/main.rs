mod models;
mod routers;
use axum::Router;
use std::env;
use crate::routers::movie_router::get_movie_routes;

const DEFAULT_HTTP_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let http_port: u16 = env::var("HTTP_PORT")
        .map(|http_port| http_port.parse::<u16>().unwrap_or(DEFAULT_HTTP_PORT))
        .unwrap_or(DEFAULT_HTTP_PORT);

    let app = Router::new()
        .nest("/api/movies", get_movie_routes());

    log::info!("Application is starting on port {}", http_port);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", http_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
