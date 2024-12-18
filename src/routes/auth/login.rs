use actix_web::{post, web, Error, HttpResponse, Responder};
use log::error;

use crate::routes::{
    auth::models::{AuthResponse, LoginRequest},
    models::{error::ErrorResponse, ModelValidator, UserResponse},
};

#[post("/login")]
async fn login(data: Result<web::Json<LoginRequest>, Error>) -> impl Responder {
    match data.validate() {
        Ok(data) => {
            let response = AuthResponse {
                user: UserResponse {
                    username: data.username,
                },
                token: "token".to_string(),
                refresh_token: "refresh_token".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => err.into(),
    }
}

impl<'a> ModelValidator<'a, LoginRequest> for Result<web::Json<LoginRequest>, Error> {
    fn validate(self) -> Result<Box<LoginRequest>, &'static ErrorResponse<'static>> {
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
