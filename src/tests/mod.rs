#[cfg(test)]
mod tests {
    use actix_http::StatusCode;
    use actix_web::{http::header::ContentType, test, App};

    use serde_json::json;

    use crate::routes::auth::login;

    #[actix_web::test]
    async fn login_success() {
        let app = test::init_service(App::new().service(login)).await;
        let req_json = json!({
            "login": "admin",
            "username": "admin_name",
            "password": "admin_password",
        });
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn login_parce_error() {
        let app = test::init_service(App::new().service(login)).await;
        let req_json = json!("{}");
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = test::read_body(resp).await;
        assert_eq!(body, r#"Failed to parse JSON"#);
    }

    #[actix_web::test]
    async fn login_empty_login() {
        let app = test::init_service(App::new().service(login)).await;
        let req_json = json!({
            "login": "",
            "username": "admin_name",
            "password": "admin_password",
        });
        let req = test::TestRequest::post()
            .set_json(req_json)
            .uri("/login")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = test::read_body(resp).await;
        assert_eq!(body, r#"Login cannot be empty"#);
    }

    #[actix_web::test]
    async fn login_empty_password() {
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
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = test::read_body(resp).await;
        assert_eq!(body, r#"Password cannot be empty"#);
    }

    #[actix_web::test]
    async fn login_empty_username() {
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
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = test::read_body(resp).await;
        assert_eq!(body, r#"Username cannot be empty"#);
    }
}
