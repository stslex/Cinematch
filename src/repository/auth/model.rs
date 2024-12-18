use uuid::Uuid;

pub struct RegistrationRequestData {
    pub username: String,
    pub login: String,
    pub password: String,
}

pub struct UserResponseData {
    pub uuid: Uuid,
    pub username: String,
}

pub enum ErrorResponseData {
    USER_ALREADY_EXISTS,
    INTERNAL_SERVER_ERROR,
    BLOCKING_ERROR,
}
