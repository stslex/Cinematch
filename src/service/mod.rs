mod api_services;
mod auth_service;
mod user_service;

pub trait ApiServices {
    fn api_v1(self) -> Self;
}

pub trait UserService {
    fn user_service(self) -> Self;
}

pub trait AuthService {
    fn auth_service(self) -> Self;
}
