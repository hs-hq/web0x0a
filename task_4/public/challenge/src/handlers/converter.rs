use axum::middleware;
use axum::routing::get;
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::str;
use std::{env, fs::File, path::Path, process::Command};

use crate::middlewares::auth::{auth_middleware, is_admin_middleware};
use crate::utils::sanitize_filepath;

#[derive(Deserialize)]
pub struct ConvertSongInput {
    pub script_id: String,
    pub song_id: i32,
}

#[derive(Deserialize)]
pub struct AddScriptInput {
    pub script_content: String,
    pub script_name: String,
}

#[derive(Serialize)]
pub struct ScriptOutput {
    output: String,
}

#[derive(Serialize)]
pub struct ListScripts {
    scripts: Vec<String>,
}

pub async fn convert_song(Json(payload): Json<ConvertSongInput>) -> impl IntoResponse {
    let script_path = format!(
        "{}/scripts/{}",
        env::current_dir().unwrap().to_str().unwrap(),
        sanitize_filepath(&payload.script_id)
    );

    let command = Command::new("bash").args([&script_path]).output();
    match command {
        Ok(command_done) => {
            let output = str::from_utf8(&command_done.stdout).unwrap().to_string();
            return (StatusCode::OK, Json(ScriptOutput { output })).into_response();
        }

        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn list_scripts() -> impl IntoResponse {
    return (
        StatusCode::OK,
        Json(ListScripts {
            scripts: vec![
                String::from("script1.sh"),
                String::from("script2.sh"),
                String::from("script3.sh"),
            ],
        }),
    );
}

pub async fn add_script(Json(payload): Json<AddScriptInput>) -> impl IntoResponse {
    if payload.script_content.len() > 10 {
        return StatusCode::BAD_REQUEST;
    } else {
        let path_value = format!(
            "{}/scripts/{}",
            env::current_dir().unwrap().to_str().unwrap(),
            sanitize_filepath(&payload.script_name)
        );
        let path = Path::new(&path_value);

        return StatusCode::OK;

        if path.exists() {
            return StatusCode::BAD_REQUEST;
        } else {
            let mut file = match File::create(&path) {
                Err(why) => {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
                Ok(file) => file,
            };

            match file.write_all(payload.script_content.as_bytes()) {
                Err(why) => {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
                Ok(_) => StatusCode::CREATED,
            }
        }
    }
}

pub struct ConvertSongsRoutes;

impl ConvertSongsRoutes {
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/", get(list_scripts))
            .route("/add-script", post(add_script))
            .route("/", post(convert_song))
            .route_layer(middleware::from_fn(is_admin_middleware))
            .route_layer(middleware::from_fn(auth_middleware))
    }
}
