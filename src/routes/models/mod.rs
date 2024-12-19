use actix_web::HttpResponse;
use error::ErrorResponse;
use serde::{Deserialize, Serialize};

pub mod error;
mod response_parser;

pub trait ModelValidator<'a, D>
where
    D: Deserialize<'a>,
{
    fn validate(self) -> Result<Box<D>, &'static ErrorResponse<'static>>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub username: String,
}

pub trait ModelOpen<R, E>
where
    R: Serialize,
{
    fn to_response(self) -> HttpResponse;
}
