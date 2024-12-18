use model::{ErrorResponseData, RegistrationRequestData, UserResponseData};

pub mod model;
mod repository;

pub trait AuthRepository {
    async fn registration(
        self,
        model: RegistrationRequestData,
    ) -> Result<UserResponseData, ErrorResponseData>;
}
