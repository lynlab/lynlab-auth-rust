extern crate biscuit;

use std;
use std::time::{SystemTime, UNIX_EPOCH};
use self::biscuit::{JWT, ClaimsSet, RegisteredClaims};
use self::biscuit::jwa::SignatureAlgorithm;
use self::biscuit::jws::{RegisteredHeader, Secret};


#[derive(Debug, Serialize, Deserialize)]
struct FirebaseClaims {
    alg: String,
    iss: String,
    sub: String,
    aud: String,
    iat: u64,
    exp: u64,
    uid: String,
}

pub fn make_firebase_token(user_id: String) -> String {
    let now_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let claims = FirebaseClaims {
        alg: "RS256".to_string(),
        iss: std::env::var("FIREBASE_SERVICE_ACCOUNT").expect("FIREBASE_SERVICE_ACCOUNT required"),
        sub: std::env::var("FIREBASE_SERVICE_ACCOUNT").unwrap(),
        aud: "https://identitytoolkit.googleapis.com/google.identity.identitytoolkit.v1.IdentityToolkit".to_string(),
        iat: now_timestamp,
        exp: now_timestamp + 3600,
        uid: user_id,
    };

    let expected_jwt = JWT::new_decoded(
        From::from(RegisteredHeader { algorithm: SignatureAlgorithm::RS256, ..Default::default() }),
        ClaimsSet::<FirebaseClaims> { registered: RegisteredClaims::default(), private: claims },
    );
    
    expected_jwt
        .into_encoded(&Secret::rsa_keypair_from_file("./secrets/firebase_key.der").unwrap())
        .unwrap()
        .unwrap_encoded()
        .to_string()
}
