#[cfg(test)]
mod tests {
    use std::ops::Not;

    use crate::{
        config::AppStateConfig,
        routes::{
            auth::{
                login::login,
                models::{AuthResponse, LoginRequest},
            },
            models::error::ErrorResponse,
        },
    };
    use actix_web::{http::header::ContentType, test, App};
    use serde_json::json;

    #[actix_web::test]
    async fn login_success() {
        let app = App::new().bind_app_state_for_tests().service(login);
        let service = test::init_service(app).await;
        let request_model = LoginRequest {
            login: "admin".to_string(),
            password: "admin_password".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&service, req).await;
        assert!(resp.status().is_success());

        let result: AuthResponse = test::read_body_json(resp).await;
        assert!(result.user.username.is_empty().not());
    }

    #[actix_web::test]
    async fn login_parce_error<'a>() {
        let expected_error = ErrorResponse::JSON_PARSE;

        let app = App::new().bind_app_state_for_tests().service(login);
        let service = test::init_service(app).await;

        let req_json = json!("{}");
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&service, req).await;

        assert_eq!(resp.status(), expected_error.status);

        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn login_empty_login() {
        let expected_error = ErrorResponse::EMPTY_LOGIN;

        let app = App::new().bind_app_state_for_tests().service(login);
        let service = test::init_service(app).await;

        let request_model = LoginRequest {
            login: "".to_string(),
            password: "admin_password".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&service, req).await;

        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn login_empty_password() {
        let expected_error = ErrorResponse::EMPTY_PASSWORD;

        let app = App::new().bind_app_state_for_tests().service(login);
        let service = test::init_service(app).await;

        let request_model = LoginRequest {
            login: "admin".to_string(),
            password: "".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&service, req).await;
        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }
}
