use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};




use serde::{Deserialize, Serialize};

use crate::{
    middlewares::auth::auth_middleware,
    repositories::artists::{Artist, ArtistUpdatable, ArtistsRepository},
};

use super::common::Error;

#[derive(Serialize)]
struct ArtistsResponse {
    artists: Vec<Artist>,
}

#[derive(Deserialize)]
struct OneArtistInput {
    artist_id: i32,
}

#[derive(Serialize)]
struct OneArtistResponse {
    artist: Artist,
}

#[derive(Deserialize)]
struct CreateArtistInput {
    pub name: String,
    pub started_at: String,
    pub description: String,
    pub origin_country: String,
}

async fn create_artist(Json(payload): Json<CreateArtistInput>) -> impl IntoResponse {
    let artists_repository = ArtistsRepository {};
    let new_artist = artists_repository.pinsert_artist(
        &payload.name,
        &payload.description,
        &payload.origin_country,
        &payload.started_at,
    );

    match new_artist {
        Ok(new_artist) => (
            StatusCode::CREATED,
            Json(OneArtistResponse { artist: new_artist }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(Error {
                message: "Error creating artist".to_string(),
            }),
        )
            .into_response(),
    }
}

async fn get_all_artists() -> impl IntoResponse {
    let artists_repository = ArtistsRepository {};

    (
        StatusCode::OK,
        Json(ArtistsResponse {
            artists: artists_repository.get_all_artists().unwrap_or(vec![]),
        }),
    )
}

async fn get_one_artist(Json(payload): Json<OneArtistInput>) -> impl IntoResponse {
    let artists_repository = ArtistsRepository {};

    let artist = artists_repository.get_artist_by_id(payload.artist_id);

    match artist {
        Ok(artist) => (StatusCode::FOUND, Json(OneArtistResponse { artist })).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(Error {
                message: "artist is not found".to_string(),
            }),
        )
            .into_response(),
    }
}

async fn delete_artist(Json(payload): Json<OneArtistInput>) -> impl IntoResponse {
    let artists_repository = ArtistsRepository {};

    match artists_repository.delete_artist_by_id(payload.artist_id) {
        true => StatusCode::MOVED_PERMANENTLY,
        false => StatusCode::NOT_FOUND,
    }
}

#[derive(Deserialize)]
struct UpdateArtist {
    pub artist_id: i32,
    pub name: Option<String>,
    pub started_at: Option<String>,
    pub description: Option<String>,
    pub origin_country: Option<String>,
}
async fn update_artist(Json(payload): Json<UpdateArtist>) -> impl IntoResponse {
    let artists_repository = ArtistsRepository {};

    let updated_user = artists_repository.supdate_artist(
        payload.artist_id,
        &ArtistUpdatable {
            name: payload.name,
            started_at: payload.started_at,
            description: payload.description,
            origin_country: payload.origin_country,
        },
    );

    match updated_user {
        Ok(updated_user) => (
            StatusCode::OK,
            Json(OneArtistResponse {
                artist: updated_user,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub struct ArtistsRoutes;

impl ArtistsRoutes {
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/", get(get_all_artists))
            .route("/one", get(get_one_artist))
            .route("/", post(create_artist))
            .route("/", patch(update_artist))
            .route("/", delete(delete_artist))
            .route_layer(middleware::from_fn(auth_middleware))
    }
}
