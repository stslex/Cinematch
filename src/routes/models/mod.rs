use error::ErrorResponse;
use serde::{Deserialize, Serialize};

pub mod error;

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
