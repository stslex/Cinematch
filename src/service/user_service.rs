use actix_web::web;
use actix_web::Scope;

use crate::routes::user_routes::get_user;

use super::UserService;

impl UserService for Scope {
    fn user_service(self) -> Self {
        self.service(web::scope("/user").service(get_user))
    }
}
