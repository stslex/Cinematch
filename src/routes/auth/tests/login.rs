#[cfg(test)]
mod tests {
    use crate::routes::{
        auth::{
            login::login,
            models::{AuthResponse, LoginRequest},
        },
        models::error::ErrorResponse,
    };
    use actix_web::{http::header::ContentType, test, App};
    use serde_json::json;

    #[actix_web::test]
    async fn login_success() {
        let app = test::init_service(App::new().service(login)).await;
        let request_model = LoginRequest {
            login: "admin".to_string(),
            username: "admin_name".to_string(),
            password: "admin_password".to_string(),
        };
        let username_clone = request_model.username.clone();
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let result: AuthResponse = test::read_body_json(resp).await;
        assert_eq!(username_clone, result.user.username);
    }

    #[actix_web::test]
    async fn login_parce_error<'a>() {
        let expected_error = ErrorResponse::JSON_PARSE;

        let app = test::init_service(App::new().service(login)).await;
        let req_json = json!("{}");
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), expected_error.status);

        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn login_empty_login() {
        let expected_error = ErrorResponse::EMPTY_LOGIN;

        let app = test::init_service(App::new().service(login)).await;
        let request_model = LoginRequest {
            login: "".to_string(),
            username: "admin_name".to_string(),
            password: "admin_password".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(request_model)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn login_empty_password() {
        let expected_error = ErrorResponse::EMPTY_PASSWORD;

        let app = test::init_service(App::new().service(login)).await;
        let req_json = json!({
            "login": "admin",
            "username": "admin_name",
            "password": "",
        });
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }

    #[actix_web::test]
    async fn login_empty_username() {
        let expected_error = ErrorResponse::EMPTY_USERNAME;

        let app = test::init_service(App::new().service(login)).await;
        let req_json = json!({
            "login": "admin",
            "username": "",
            "password": "admin_password",
        });
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), expected_error.status);
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_error.cause);
    }
}
