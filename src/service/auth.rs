use actix_web::{post, web, HttpResponse, Responder};

pub fn auth_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(login));
}

#[post("/")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("wow login!")
}
