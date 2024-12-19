-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table IF NOT EXISTS users (
    uuid uuid default uuid_generate_v4 () not null constraint table_name_pk primary key,
    login varchar(123) not null,
    username varchar(128) not null,
    secret text not null
);

create unique index IF NOT EXISTS users_uuid_uindex on users (uuid);

create unique index IF NOT EXISTS users_login_uindex on users (login);

create unique index IF NOT EXISTS users_username_uindex on users (username);

create unique index IF NOT EXISTS users_secret_uindex on users (secret);
