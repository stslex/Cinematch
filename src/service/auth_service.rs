use actix_web::{post, web, HttpResponse, Responder, Scope};

use super::AuthService;

impl AuthService for Scope {
    fn auth_service(self) -> Self {
        self.service(web::scope("/auth").service(login))
    }
}

#[post("/")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("wow login!")
}
