use actix_web::web;

use crate::routes::auth::{login::login, registration::registration};

pub fn auth_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(login).service(registration));
}
