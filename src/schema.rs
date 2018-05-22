table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password_hash -> Varbinary,
        password_salt -> Varchar,
        email -> Varchar,
        access_token -> Nullable<Varchar>,
        access_token_valid_until -> Nullable<Datetime>,
        is_activated -> Bool,
        activate_token -> Nullable<Varchar>,
        activate_token_valid_until -> Nullable<Datetime>,
    }
}
