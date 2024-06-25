use axum::Router;

use self::{
    artists::ArtistsRoutes, auth::AuthRoutes, converter::ConvertSongsRoutes, songs::SongsRoutes,
};

pub mod artists;
pub mod auth;
pub mod common;
pub mod converter;
pub mod songs;
pub mod static_files;

pub fn create_router() -> Router {
    let songs_router = SongsRoutes {};
    let auth_router = AuthRoutes {};
    let producers_router = ArtistsRoutes {};
    let converter_router = ConvertSongsRoutes {};

    Router::new()
        .nest("/auth", auth_router.routes())
        .nest("/artists", producers_router.routes())
        .nest("/songs", songs_router.routes())
        .nest("/convert", converter_router.routes())
        .nest("/static", static_files::two_serve_dirs())
}
