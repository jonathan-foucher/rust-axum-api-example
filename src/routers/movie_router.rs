use axum::{extract::{Path, Json}, http::StatusCode, routing::{delete, get, post}, Router};
use crate::models::movie::Movie;

pub fn get_movie_routes() -> Router {
    Router::new()
        .route("/", get(get_all_movies))
        .route("/", post(save_movie))
        .route("/:movieId", delete(delete_movie))
}

async fn get_all_movies() -> (StatusCode, &'static str) {
    log::info!("Get all movies");
    (StatusCode::OK, "Get all movies")
}

async fn save_movie(Json(body): Json<Movie>) -> StatusCode {
    log::info!("Post movie id={}, title='{}' and relase_date={}", body.id, body.title, body.release_date);
    StatusCode::OK
}

async fn delete_movie(Path(movie_id): Path<i32>) -> StatusCode {
    log::info!("Delete movie with id {}", movie_id);
    StatusCode::OK
}
