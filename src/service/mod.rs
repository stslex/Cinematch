use crate::config::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use auth::auth_service;
use user::user_service;

mod auth;
mod user;

pub fn api_v1_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(user_service)
            .configure(auth_service)
            .service(hello),
    );
}

#[get("")]
async fn hello<'a>(data: web::Data<AppState<'a>>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("app running {}", app_name))
}
