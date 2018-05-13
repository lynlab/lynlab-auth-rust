table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password_hash -> Varbinary,
        password_salt -> Varchar,
        email -> Varchar,
        access_token -> Nullable<Varchar>,
        is_activated -> Bool,
    }
}
