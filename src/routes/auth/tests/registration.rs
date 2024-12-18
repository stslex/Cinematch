#[cfg(test)]
mod tests {
    use crate::{
        config::AppStateConfig,
        routes::{
            auth::{
                models::{AuthResponse, RegistrationRequest},
                registration::registration,
            },
            models::error::ErrorResponse,
        },
    };
    use actix_web::{http::header::ContentType, test, App};
    use serde_json::json;

    #[actix_web::test]
    async fn registration_success() {
        let app = App::new().bind_app_state_for_tests().service(registration);
        let service = test::init_service(app).await;

        let request_model = RegistrationRequest {
            login: "admin".to_string(),
            username: "admin_name".to_string(),
            password: "admin_password".to_string(),
        };
        let request_username = request_model.username.clone();
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/registration")
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&service, req).await;
        assert!(resp.status().is_success());

        let result: AuthResponse = test::read_body_json(resp).await;
        assert_eq!(request_username, result.user.username);
    }

    #[actix_web::test]
    async fn registration_parce_error<'a>() {
        let expected_error = ErrorResponse::JSON_PARSE;

        let app = App::new().bind_app_state_for_tests().service(registration);
        let service = test::init_service(app).await;

        let req_json = json!("{}");
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/registration")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&service, req).await;

        assert_eq!(resp.status(), expected_error.status);

        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn registration_empty_login() {
        let expected_error = ErrorResponse::EMPTY_LOGIN;

        let app = App::new().bind_app_state_for_tests().service(registration);
        let service = test::init_service(app).await;

        let request_model = RegistrationRequest {
            login: "".to_string(),
            username: "admin_name".to_string(),
            password: "admin_password".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/registration")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&service, req).await;

        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn registration_empty_password() {
        let expected_error = ErrorResponse::EMPTY_PASSWORD;

        let app = App::new().bind_app_state_for_tests().service(registration);
        let service = test::init_service(app).await;

        let request_model = RegistrationRequest {
            login: "admin".to_string(),
            username: "admin_name".to_string(),
            password: "".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/registration")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&service, req).await;
        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn registration_empty_username() {
        let expected_error = ErrorResponse::EMPTY_USERNAME;

        let app = App::new().bind_app_state_for_tests().service(registration);
        let service = test::init_service(app).await;

        let request_model = RegistrationRequest {
            login: "admin".to_string(),
            username: "".to_string(),
            password: "password".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/registration")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&service, req).await;
        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }
}
