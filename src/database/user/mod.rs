use model::{UserCreateEntity, UserEntity};
use uuid::Uuid;

use super::ErrorResponseDb;

mod database;
pub mod model;

pub trait UserDatabase {
    fn create_user(self, user: UserCreateEntity) -> Result<UserEntity, ErrorResponseDb>;
    fn get_user(self, uuid: Uuid) -> Result<UserEntity, ErrorResponseDb>;
    fn get_user_by_login(self, login: String) -> Result<UserEntity, ErrorResponseDb>;
    fn get_user_by_username(self, username: String) -> Result<UserEntity, ErrorResponseDb>;
}
