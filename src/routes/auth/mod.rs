use crate::routes::models::{ErrorResponse, UserResponse};
use actix_web::{post, web, Error, HttpResponse, Responder};
use log::error;
use models::{AuthResponse, LoginRequest};

use super::models::ModelValidator;

mod models;

#[post("/login")]
async fn login(data: Result<web::Json<LoginRequest>, Error>) -> impl Responder {
    match data.validate() {
        Ok(data) => {
            let response = AuthResponse {
                user: UserResponse {
                    username: &data.username,
                },
                token: "token",
                refresh_token: "refresh_token",
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => err.into(),
    }
}

impl<'a> ModelValidator<'a, LoginRequest> for Result<web::Json<LoginRequest>, Error> {
    fn validate(self) -> Result<Box<LoginRequest>, ErrorResponse<'static>> {
        match self {
            Ok(data) => {
                if data.login.is_empty() {
                    return Err(ErrorResponse::EMPTY_LOGIN);
                }
                if data.password.is_empty() {
                    return Err(ErrorResponse::EMPTY_PASSWORD);
                }
                if data.username.is_empty() {
                    return Err(ErrorResponse::EMPTY_USERNAME);
                }
                Ok(Box::new(data.into_inner()))
            }
            Err(err) => {
                error!("Failed to parse JSON: {}", err);
                Err(ErrorResponse::JSON_PARSE)
            }
        }
    }
}
