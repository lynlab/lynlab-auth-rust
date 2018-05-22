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
        activation_token -> Nullable<Varchar>,
        activation_token_valid_until -> Nullable<Datetime>,
        activation_redirection_url -> Nullable<Varchar>,
    }
}
