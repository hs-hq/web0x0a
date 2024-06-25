use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    handlers::common::UserStruct, repositories::users::UsersRepository, utils::retrieve_user_file,
};

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) => {
            let auth_token: Vec<&str> = auth_header.split(" ").collect();

            let user_token = auth_token[1];

            let user_struct = retrieve_user_file(user_token);

            match user_struct {
                Ok(user_struct) => {
                    req.extensions_mut().insert(user_struct);
                    Ok(next.run(req).await)
                }
                Err(error) => Err(StatusCode::UNAUTHORIZED),
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn is_admin_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let user = req.extensions_mut().get::<UserStruct>();
    let users_repository = UsersRepository {};

    match user {
        Some(user_struct) => {
            let user = users_repository.get_user_by_username(user_struct.username.as_str());
            match user {
                Ok(user) => {
                    if user.role == "ADMIN" {
                        return Ok(next.run(req).await);
                    } else {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
