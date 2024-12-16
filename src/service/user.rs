use actix_web::web;

use crate::routes::user_routes::get_user;

pub fn user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(get_user));
}
