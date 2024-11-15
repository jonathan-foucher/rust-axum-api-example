use axum::{extract, extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Json, Router};
use diesel::{insert_into, pg::PgConnection, prelude::*, r2d2::{ConnectionManager, Pool}, upsert::excluded};
use crate::models::movie::Movie;
use crate::schema::movie::dsl::*;

fn internal_error<E>(error: E) -> (StatusCode, String)
where E: std::error::Error,
{
    log::error!("{error}");
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

pub fn get_movie_routes() -> Router<Pool<ConnectionManager<PgConnection>>> {
    Router::new()
        .route("/", get(get_all_movies))
        .route("/", post(save_movie))
        .route("/:movieId", delete(delete_movie))
}

async fn get_all_movies(
    State(db_pool): State<Pool<ConnectionManager<PgConnection>>>
) -> Result<Json<Vec<Movie>>, (StatusCode, String)> {
    log::info!("Get all movies");
    let mut db_conn = db_pool.get().expect("Could not get a database connection from the pool");
    let results: Vec<Movie> = movie.load(&mut db_conn)
        .map_err(internal_error)?;
    Ok(Json(results))
}

async fn save_movie(
    State(db_pool): State<Pool<ConnectionManager<PgConnection>>>,
    extract::Json(body): Json<Movie>
) -> Result<Json<Movie>, (StatusCode, String)> {
    log::info!("Post movie id={}, title='{}' and relase_date={}", body.id, body.title, body.release_date);
    let mut db_conn = db_pool.get().expect("Could not get a database connection from the pool");

    let movie_id: i32 = body.id;
    let _ = insert_into(movie).values(body)
        .on_conflict(id)
        .do_update()
        .set((
                title.eq(excluded(title)),
                release_date.eq(excluded(release_date))
            ))
        .execute(&mut db_conn)
        .map_err(internal_error)?;

    let result: Movie = movie.filter(id.eq(movie_id)).first(&mut db_conn)
        .map_err(internal_error)?;
    Ok(Json(result))
}

async fn delete_movie(
    Path(movie_id): Path<i32>,
    State(db_pool): State<Pool<ConnectionManager<PgConnection>>>
) -> Result<StatusCode,(StatusCode, String)> {
    log::info!("Delete movie with id {}", movie_id);
    let mut db_conn = db_pool.get().expect("Could not get a database connection from the pool");
    let result = diesel::delete(movie.filter(id.eq(movie_id))).execute(&mut db_conn)
        .map_err(internal_error)?;

    if result == 0 { Ok(StatusCode::NO_CONTENT) } else { Ok(StatusCode::OK) }
}
