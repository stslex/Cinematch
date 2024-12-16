use actix_web::{get, web, HttpResponse, Responder, Scope};

use super::{ApiServices, AuthService, UserService};

impl ApiServices for Scope {
    fn api_v1(self) -> Self {
        self.service(
            web::scope("/v1")
                .user_service()
                .auth_service()
                .service(hello),
        )
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
