use std::io;

use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use tower_http::services::ServeDir;

pub fn two_serve_dirs() -> Router {
    // you can also have two `ServeDir`s nested at different paths
    let serve_dir_from_covers =
        get_service(ServeDir::new("static/covers")).handle_error(handle_error);
    let serve_dir_from_songs =
        get_service(ServeDir::new("static/songs")).handle_error(handle_error);

    Router::new()
        .nest_service("/covers", serve_dir_from_covers)
        .nest_service("/songs", serve_dir_from_songs)
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
