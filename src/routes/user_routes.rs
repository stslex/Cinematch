use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Hello world get user!")
}
