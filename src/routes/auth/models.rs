use actix_http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::routes::models::{ErrorResponse, UserResponse};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse<'a> {
    pub user: UserResponse<'a>,
    pub token: &'a str,
    pub refresh_token: &'a str,
}

struct RegistrationRequest<'a> {
    login: &'a str,
    password: &'a str,
    username: &'a str,
}

impl ErrorResponse<'static> {
    pub const EMPTY_LOGIN: ErrorResponse<'static> = ErrorResponse {
        cause: "Login cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
    pub const EMPTY_PASSWORD: ErrorResponse<'static> = ErrorResponse {
        cause: "Password cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
    pub const EMPTY_USERNAME: ErrorResponse<'static> = ErrorResponse {
        cause: "Username cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
}
