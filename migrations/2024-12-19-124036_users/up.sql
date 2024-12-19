-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    uuid UUID DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT users_pk PRIMARY KEY,
    login VARCHAR(123) NOT NULL,
    username VARCHAR(128) NOT NULL,
    secret TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS users_uuid_uindex ON users (uuid);
CREATE UNIQUE INDEX IF NOT EXISTS users_login_uindex ON users (login);
CREATE UNIQUE INDEX IF NOT EXISTS users_username_uindex ON users (username);
CREATE UNIQUE INDEX IF NOT EXISTS users_secret_uindex ON users (secret);
