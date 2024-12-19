use diesel::{result::DatabaseErrorKind, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use log::error;
use uuid::Uuid;

use crate::{database::ErrorResponseDb, schema::users};

use super::{
    model::{UserCreateEntity, UserEntity},
    UserDatabase,
};

impl UserDatabase for &mut PgConnection {
    fn create_user(self, user: UserCreateEntity) -> Result<UserEntity, ErrorResponseDb> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result::<UserEntity>(self)
            .map_err(|err| {
                error!("Failed to create user: {}", err);
                match err {
                    diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        ErrorResponseDb::Conflict
                    }
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }

    fn get_user(self, uuid: Uuid) -> Result<UserEntity, ErrorResponseDb> {
        users::table
            .filter(users::uuid.eq(uuid))
            .first::<UserEntity>(self)
            .map_err(|err| {
                error!("Failed to get user by login: {}", err);
                match err {
                    diesel::result::Error::NotFound => ErrorResponseDb::NotFound,
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }

    fn get_user_by_login(self, login: String) -> Result<UserEntity, ErrorResponseDb> {
        users::table
            .filter(users::login.eq(login))
            .first::<UserEntity>(self)
            .map_err(|err| {
                error!("Failed to get user by login: {}", err);
                match err {
                    diesel::result::Error::NotFound => ErrorResponseDb::NotFound,
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }

    fn get_user_by_username(self, username: String) -> Result<UserEntity, ErrorResponseDb> {
        users::table
            .filter(users::username.eq(username))
            .first::<UserEntity>(self)
            .map_err(|err| {
                error!("Failed to get user by username: {}", err);
                match err {
                    diesel::result::Error::NotFound => ErrorResponseDb::NotFound,
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }
}
