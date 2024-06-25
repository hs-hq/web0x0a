use axum::{
    extract::{multipart::Field, Multipart, Path},
    http::{header, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    middlewares::auth::auth_middleware,
    repositories::{
        artists::ArtistsRepository,
        songs::{Song, SongUpdatable, SongsRepository},
    },
    utils::sanitize_filepath,
};

enum FileUploadResult {
    BadFormat(String),
    FileTooBig(String),
    FileSaved(String, String),
    FileNotSaved(String),
}

use FileUploadResult::*;

use super::common::Error;

const MAX_UPLOAD_SIZE: usize = 1024 * 1024 * 10;

#[derive(Serialize)]
struct SongsResponse {
    songs: Vec<Song>,
}

#[derive(Serialize)]
struct OneSongResponse {
    song: Song,
}

#[derive(Deserialize)]
struct OneSongInput {
    pub song_id: Option<i32>,
    pub song_name: Option<String>,
}

// #[derive(Deserialize)]
// struct OneSongId {
//     pub song_id: i32,
// }
#[derive(Deserialize)]
struct ProducersSong {
    artist_id: Option<i32>,
    song_name: Option<String>,
}

async fn get_all_songs() -> impl IntoResponse {
    let SongsRepository = SongsRepository {};

    let songs = SongsRepository.get_all_songs();
    (
        StatusCode::OK,
        Json(SongsResponse {
            songs: songs.unwrap_or(vec![]),
        }),
    )
}

async fn get_one_song(Json(payload): Json<OneSongInput>) -> impl IntoResponse {
    let SongsRepository = SongsRepository {};

    if payload.song_id.is_some() {
        return match payload.song_id {
            Some(song_id) => SongsRepository.get_song_by_id(song_id).map_or_else(
                |_| StatusCode::NOT_FOUND.into_response(),
                |song| (StatusCode::FOUND, Json(OneSongResponse { song })).into_response(),
            ),
            None => StatusCode::NOT_FOUND.into_response(),
        };
    } else if payload.song_name.is_some() {
        let song_name = payload.song_name.unwrap_or("".to_string());
        if song_name.is_empty() {
            return (
                StatusCode::NOT_FOUND,
                Json(Error {
                    message: "Make sure to provide a valid name next time ;)".to_string(),
                }),
            )
                .into_response();
        }
        return SongsRepository.get_song_by_name(&song_name).map_or_else(
            |_| (StatusCode::NOT_FOUND).into_response(),
            |song| (StatusCode::FOUND, Json(OneSongResponse { song })).into_response(),
        );
    } else {
        return (StatusCode::NOT_FOUND).into_response();
    }
}

#[derive(Deserialize)]
struct CreateSong {
    pub song_name: String,
    pub date_of_release: String,
    pub song_cover: String,
    pub song_file: String,
    pub artist_id: i32,
}

#[derive(Deserialize)]
struct UpdateSong {
    pub song_name: Option<String>,
    pub date_of_release: Option<String>,
    pub song_cover: Option<String>,
    pub song_file: Option<String>,
    pub artist_id: Option<i32>,
}

async fn create_song(Json(payload): Json<CreateSong>) -> impl IntoResponse {
    let artists_repository = ArtistsRepository {};
    let artist = artists_repository.get_artist_by_id(payload.artist_id);
    if artist.is_ok() {
        let songs_repository = SongsRepository {};

        let new_song = songs_repository.pinsert_song(
            &payload.song_name,
            &payload.date_of_release,
            &payload.song_file,
            &payload.song_cover,
            payload.artist_id,
        );
        return match new_song {
            Ok(created_song) => (
                StatusCode::CREATED,
                Json(OneSongResponse { song: created_song }),
            )
                .into_response(),
            Err(_) => StatusCode::BAD_REQUEST.into_response(),
        };
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(Error {
                message: "Producer does not exist".to_string(),
            }),
        )
            .into_response();
    }
}

async fn delete_song(Path(id): Path<i32>) -> impl IntoResponse {
    let songs_repository = SongsRepository {};
    let song_deleted = songs_repository.delete_song_by_id(id);

    match song_deleted {
        true => StatusCode::OK,
        false => StatusCode::NOT_FOUND,
    }
}

async fn update_song(Path(id): Path<i32>, Json(payload): Json<UpdateSong>) -> impl IntoResponse {
    let songs_repository = SongsRepository;
    let updated_song = SongUpdatable {
        date_of_release: payload.date_of_release,
        name: payload.song_name,
        artist_id: payload.artist_id,
        song_cover: payload.song_cover,
        song_file: payload.song_file,
    };
    songs_repository
        .supdate_song(id, &updated_song)
        .map_or_else(
            |_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(Error {
                        message: "Please make sure your request is correct".to_string(),
                    }),
                )
                    .into_response()
            },
            |updated_song| {
                (StatusCode::OK, Json(OneSongResponse { song: updated_song })).into_response()
            },
        )
}
// For testing purposes

async fn upload_song_files(mut multipart: Multipart) -> impl IntoResponse {
    let mut i = 0;

    while let Some(file) = multipart.next_field().await.unwrap() {
        let name = file.name().unwrap().to_string();

        if name.eq("song_file") {
            if let Err(_) = upload_song_file(file).await {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Error {
                        message: String::from("Error creating file."),
                    }),
                )
                    .into_response();
            }
        } else if name.eq("cover_image") {
            if let Err(_) = upload_cover_file(file).await {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Error {
                        message: String::from("Error creating file"),
                    }),
                )
                    .into_response();
            }
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }

        i += 1;
        if i == 2 {
            break;
        }
    }
    return StatusCode::OK.into_response();
}

async fn upload_cover_file(file: Field<'_>) -> Result<FileUploadResult, FileUploadResult> {
    let filename = file.file_name().unwrap().to_string();
    if !filename.ends_with(".jpg") && !filename.ends_with(".png") || filename.ends_with(".webp") {
        return Err(BadFormat(String::from(
            "File format is not supported, please use jpg or png or webp.",
        )));
    }

    let data = file.bytes().await.unwrap();

    if data.len() > MAX_UPLOAD_SIZE {
        return Err(FileTooBig("File is too big".to_string()));
    }

    let file_path = format!("static/covers/{}", sanitize_filepath(&filename));

    let write_result = tokio::fs::write(&file_path, &data)
        .await
        .map_err(|err| err.to_string());

    match write_result {
        Ok(_) => Ok(FileSaved(filename, file_path)),
        Err(_) => Err(FileNotSaved("File wasn't saved".to_string())),
    }
}

async fn upload_song_file(file: Field<'_>) -> Result<FileUploadResult, FileUploadResult> {
    let filename = file.file_name().unwrap().to_string();
    if !filename.ends_with(".mp3") {
        println!("File format is not supported, please use mp3.");
        return Err(BadFormat(String::from(
            "File format is not supported, please use mp3.",
        )));
    }

    let data = file.bytes().await.unwrap();

    if data.len() > MAX_UPLOAD_SIZE {
        println!("File is too big");
        return Err(FileTooBig("File is too big".to_string()));
    }

    let file_path = format!("static/songs/{}", filename);

    let write_result = tokio::fs::write(&file_path, &data)
        .await
        .map_err(|err| err.to_string());

    println!("Writing error");
    match write_result {
        Ok(_) => Ok(FileSaved(filename, file_path)),
        Err(_) => Err(FileNotSaved("File wasn't saved".to_string())),
    }
}

async fn cn(msg: String) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain;charset=utf-8")],
        msg,
    )
        .into_response()
}

pub struct SongsRoutes;

impl SongsRoutes {
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/", get(get_all_songs))
            .route("/one", post(get_one_song))
            .route("/", post(create_song))
            .route("/:id", patch(update_song))
            .route("/:id", delete(delete_song))
            .route("/upload", post(upload_song_files))
            .route_layer(middleware::from_fn(auth_middleware))
    }
}
