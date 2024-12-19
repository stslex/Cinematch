use actix_web::{post, web, Error, Responder};
use log::error;

use crate::{
    config::database::DbPool,
    repository::auth::{
        model::{ErrorResponseData, RegistrationRequestData},
        AuthRepository,
    },
    routes::{
        auth::models::{AuthResponse, RegistrationRequest},
        models::{error::ErrorResponse, ModelOpen, ModelValidator, UserResponse},
    },
};

#[post("/registration")]
async fn registration(
    pool: web::Data<DbPool>,
    data: Result<web::Json<RegistrationRequest>, Error>,
) -> impl Responder {
    match data.validate() {
        Ok(data) => pool
            .registration(RegistrationRequestData {
                login: data.login,
                password: data.password,
                username: data.username,
            })
            .await
            .map(|response| AuthResponse {
                user: UserResponse {
                    username: response.user.username,
                },
                token: response.token,
                refresh_token: response.refresh_token,
            })
            .map_err(|err| match err {
                ErrorResponseData::USER_ALREADY_EXISTS => ErrorResponse::USER_ALREADY_EXISTS,
                ErrorResponseData::INTERNAL_SERVER_ERROR => ErrorResponse::INTERNAL_SERVER_ERROR,
                ErrorResponseData::BLOCKING_ERROR => ErrorResponse::INTERNAL_SERVER_ERROR,
            })
            .to_response(),
        Err(err) => err.into(),
    }
}

impl<'a> ModelValidator<'a, RegistrationRequest> for Result<web::Json<RegistrationRequest>, Error> {
    fn validate(self) -> Result<Box<RegistrationRequest>, &'static ErrorResponse<'static>> {
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
