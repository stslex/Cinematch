use actix_web::web;
pub mod sert;

pub struct AppState<'a> {
    pub app_name: &'a str,
}

pub trait ServiceSertConfig {
    fn bind_server_ssl(self) -> Self;
}

pub fn configure_data(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(AppState {
        app_name: "Cinematch",
    }));
}
