use actix_web::{web, App, HttpServer};

use config::ServiceSertConfig;
use env_logger::Env;
use log::info;
use service::api_v1_service;
mod config;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .configure(config::configure_data)
            .service(web::scope("/api").configure(api_v1_service))
    })
    .bind_server_ssl()
    .run()
    .await
}
