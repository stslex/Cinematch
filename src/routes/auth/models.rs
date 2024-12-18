use actix_http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::routes::models::{error::ErrorResponse, UserResponse};

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegistrationRequest {
    pub login: String,
    pub username: String,
    pub password: String,
}

impl ErrorResponse<'static> {
    pub const EMPTY_LOGIN: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Login cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
    pub const EMPTY_PASSWORD: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Password cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
    pub const EMPTY_USERNAME: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Username cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
}
