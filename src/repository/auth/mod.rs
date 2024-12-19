use model::{ErrorResponseData, RegistrationRequestData, RegistrationResponseData};

pub mod model;
mod repository;

pub trait AuthRepository {
    async fn registration(
        self,
        model: RegistrationRequestData,
    ) -> Result<RegistrationResponseData, ErrorResponseData>;
}
