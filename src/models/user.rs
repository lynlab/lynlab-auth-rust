use argon2rs;
use nanoid;

use schema::users;

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: Vec<u8>,
    pub password_salt: String,
    pub email: String,
    pub access_token: Option<String>,
    pub is_activated: bool,
}

impl User {
    /// Make password hash.
    /// Return a tuple: (hashed password, salt)
    pub fn make_password_hash(password: &str) -> (Vec<u8>, String) {
        let salt = nanoid::generate(32);
        let hash = argon2rs::argon2i_simple(password, &salt).to_vec();
        
        (hash, salt.to_string())
    }
}
