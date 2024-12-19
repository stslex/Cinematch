use actix_web::HttpResponse;
use serde::Serialize;

use super::{error::ErrorResponse, ModelOpen};

impl<R> ModelOpen<R, &'static ErrorResponse<'static>> for Result<R, &'static ErrorResponse<'static>>
where
    R: Serialize,
{
    fn to_response(self) -> HttpResponse {
        match self {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(error) => error.into(),
        }
    }
}
