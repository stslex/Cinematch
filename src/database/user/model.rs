use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct UserCreateEntity {
    #[diesel(column_name = "login")]
    pub login: String,
    #[diesel(column_name = "username")]
    pub username: String,
    #[diesel(column_name = "secret")]
    pub secret: String,
}

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct UserEntity {
    #[diesel(column_name = "uuid")]
    pub uuid: Uuid,
    #[diesel(column_name = "login")]
    pub login: String,
    #[diesel(column_name = "username")]
    pub username: String,
    #[diesel(column_name = "secret")]
    pub secret: String,
}
