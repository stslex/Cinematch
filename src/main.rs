use actix_web::{web, App, HttpServer};
use service::ApiServices;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").api_v1()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
