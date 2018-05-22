use argon2rs;
use chrono::NaiveDateTime;
use nanoid;

use schema::users;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: Vec<u8>,
    pub password_salt: String,
    pub email: String,
    pub access_token: Option<String>,
    pub access_token_valid_until: Option<NaiveDateTime>,
    pub is_activated: bool,
    pub activation_token: Option<String>,
    pub activation_token_valid_until: Option<NaiveDateTime>,
    pub activation_redirection_url: Option<String>,
}

impl User {
    /// Make password hash.
    /// Return a tuple: (hashed password, salt)
    pub fn make_password_hash(password: &str) -> (Vec<u8>, String) {
        let salt = nanoid::generate(32);
        let hash = argon2rs::argon2i_simple(password, &salt).to_vec();
        
        (hash, salt.to_string())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let hash = argon2rs::argon2i_simple(password, &self.password_salt).to_vec();
        self.password_hash == hash
    }
}
