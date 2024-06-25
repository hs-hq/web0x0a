use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use std::time::SystemTime;

use std::str;

use pwhash::bcrypt;

use serde::{Deserialize, Serialize};

use crate::{middlewares::auth::auth_middleware, repositories::users::UsersRepository};

use super::common::{Error, UserStruct};

use crate::utils::save_user;

#[derive(Serialize)]
struct HealthCheck {
    message: String,
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
    time: String,
}

#[derive(Deserialize)]
struct HealthCheckRequest {
    status: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: i32,
    username: String,
}

#[derive(Deserialize)]
struct LoginUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
struct RegisterUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

async fn healthcheck() -> impl IntoResponse {
    let now = SystemTime::now();
    Json(HealthCheck {
        message: format!(
            "OK {}",
            now.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ),
    })
}

#[derive(Serialize)]
struct LoginResponse {
    id_token: String,
}
async fn login(Json(payload): Json<LoginUserInput>) -> impl IntoResponse {
    if payload.username.is_none() && payload.email.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Error {
                message: "Username or email is required".to_string(),
            }),
        )
            .into_response();
    }
    if payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Error {
                message: "Password is required".to_string(),
            }),
        )
            .into_response();
    }

    let users_repository = UsersRepository {};

    let user = match payload.username {
        Some(username) if !username.is_empty() => users_repository.get_user_by_username(&username),
        _ => match payload.email {
            Some(email) if !email.is_empty() => users_repository.get_user_by_email(&email),
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Error {
                        message: "Username or email is required".to_string(),
                    }),
                )
                    .into_response();
            }
        },
    };

    if user.is_err() {
        return (
            StatusCode::NOT_FOUND,
            Json(User {
                id: -10,
                username: "User not found".to_string(),
            }),
        )
            .into_response();
    } else {
        let user = user.unwrap();
        if validate_password(&payload.password, &user.password) {
            let user_struct = UserStruct {
                email: user.email,
                username: user.username.clone(),
            };
            let user_token = save_user(&user_struct).unwrap_or_default();
            return (
                StatusCode::OK,
                Json(LoginResponse {
                    id_token: user_token,
                }),
            )
                .into_response();
        } else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(Error {
                    message: "Invalid credentials".to_string(),
                }),
            )
                .into_response();
        }
    }
}

fn validate_password(input_password: &str, user_password: &str) -> bool {
    bcrypt::verify(input_password, user_password)
}

async fn register(Json(payload): Json<RegisterUserInput>) -> impl IntoResponse {
    let users_repository = UsersRepository {};

    // validate_register_input(&payload);

    let user_password = bcrypt::hash(payload.password);

    match user_password {
        Ok(user_password) => {
            let created_user = users_repository.pinsert_user(
                &payload.username,
                &user_password,
                &payload.email,
                "USER",
            );

            let response = StatusCode::CREATED.into_response();

            match created_user {
                Ok(_) => response,
                Err(_) => (
                    StatusCode::BAD_REQUEST,
                    Json(Error {
                        message: String::from("Error creating user"),
                    }),
                )
                    .into_response(),
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Serialize)]
struct MeResponse {
    id: i32,
    username: String,
    email: String,
    role: String,
}

async fn me(Extension(user_struct): Extension<UserStruct>) -> impl IntoResponse {
    let user_repository = UsersRepository {};

    let user = user_repository.get_user_by_username(user_struct.username.as_str());
    match user {
        Ok(user) => (
            StatusCode::OK,
            Json(MeResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                role: user.role,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}

pub struct AuthRoutes;

impl AuthRoutes {
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/healthcheck", get(healthcheck))
            .route("/login", post(login))
            .route("/register", post(register))
            .route(
                "/me",
                get(me).route_layer(middleware::from_fn(auth_middleware)),
            )
    }
}
