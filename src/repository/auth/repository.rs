use actix_web::web;
use log::error;

use super::{
    model::{ErrorResponseData, RegistrationRequestData, UserResponseData},
    AuthRepository,
};
use crate::{
    config::{DataPoolConnection, DbPool},
    database::user::{model::UserCreateEntity, UserDatabase},
};

impl AuthRepository for web::Data<DbPool> {
    async fn registration(
        self,
        model: RegistrationRequestData,
    ) -> Result<UserResponseData, ErrorResponseData> {
        self.safely_run(|conn| {
            let create_model = UserCreateEntity {
                username: model.username,
                login: model.login,
                secret: model.password,
            };
            conn.create_user(create_model)
                .map_err(|e| {
                    error!("Failed to create user: {}", e);
                    ErrorResponseData::INTERNAL_SERVER_ERROR
                })
                .map(|user| UserResponseData {
                    uuid: user.uuid,
                    username: user.username,
                })
        })
        .await
    }
}
