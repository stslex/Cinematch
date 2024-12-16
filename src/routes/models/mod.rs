use actix_http::StatusCode;
use actix_web::HttpResponse;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

pub trait ModelValidator<'a, D>
where
    D: Deserialize<'a>,
{
    fn validate(self) -> Result<Box<D>, ErrorResponse<'static>>
    where
        Self: Sized;
}

#[derive(Copy, Clone)]
pub struct ErrorResponse<'a> {
    pub cause: &'a str,
    pub status: StatusCode,
}

impl Serialize for ErrorResponse<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("ErrorResponse", 2)?;
        state.serialize_field("cause", &self.cause)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.end()
    }
}

impl ErrorResponse<'static> {
    pub const OTHER: ErrorResponse<'static> = ErrorResponse {
        cause: "An unknown error occurred",
        status: StatusCode::INTERNAL_SERVER_ERROR,
    };
    pub const JSON_PARSE: ErrorResponse<'static> = ErrorResponse {
        cause: "Failed to parse JSON",
        status: StatusCode::BAD_REQUEST,
    };
    pub const TOKEN_INVALID: ErrorResponse<'static> = ErrorResponse {
        cause: "Invalid token",
        status: StatusCode::UNAUTHORIZED,
    };
}

impl Into<actix_web::HttpResponse> for ErrorResponse<'static> {
    fn into(self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status).body(self.cause)
    }
}

impl<'a> ErrorResponse<'a> {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(self)
    }
}

#[derive(Serialize)]
pub struct UserResponse<'a> {
    pub username: &'a str,
}
