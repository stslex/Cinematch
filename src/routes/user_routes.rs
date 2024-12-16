use actix_web::{get, web, HttpResponse};

#[get("/{user_id}")]
async fn get_user(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("Hello user: {}", user_id).to_string())
}
