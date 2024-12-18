diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        login -> Varchar,
        username -> Varchar,
        secret -> Text,
    }
}
