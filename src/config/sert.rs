use actix_http::{body::MessageBody, Request, Response};
use actix_service::{IntoServiceFactory, ServiceFactory};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::fmt;

use actix_web::{dev::AppConfig, Error, HttpServer};

use super::ServiceSertConfig;

impl<F, I, S, B> ServiceSertConfig for HttpServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig> + 'static,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    S::Service: 'static,
    B: MessageBody + 'static,
{
    fn bind_server_ssl(self) -> Self {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();
        self.bind_openssl("127.0.0.1:8080", builder)
            .expect("Failed to bind ssl to address")
    }
}
