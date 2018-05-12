#![feature(custom_attribute)]

extern crate actix;
extern crate actix_web;
extern crate argon2rs;
#[macro_use] extern crate diesel;
extern crate futures;
extern crate json;
extern crate nanoid;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate uuid;

pub mod helpers;
pub mod models;
pub mod schema;

use actix_web::{server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use diesel::result::Error::DatabaseError;
use diesel::prelude::*;
use futures::Future;
use models::user::User;


struct AppState {
    pool: helpers::db::Pool,
}

#[derive(Debug, Deserialize)]
struct RegisterReqBody {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct ErrorResBody {
    error: String,
}

/// Pong.
fn ping(_: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

/// Register a new user account.
fn register(req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let conn = req.state().pool.clone().get().unwrap();
    req.json().from_err()
        .and_then(move |body: RegisterReqBody| {
            let (hash, salt) = User::make_password_hash(&body.password);
            let user = User {
                id: uuid::Uuid::new_v4().hyphenated().to_string(),
                username: body.username,
                password_hash: hash,
                password_salt: salt,
                email: body.email,
                access_token: None,
                is_activated: false,
            };
            
            let insert_result = diesel::insert_into(schema::users::table)
                .values(&user)
                .execute(&*conn);

            match insert_result {
                Ok(_) => Ok(HttpResponse::Ok().body("Ok")),
                Err(e) => match e {
                    DatabaseError(_, _) => Ok(HttpResponse::BadRequest().json(ErrorResBody { error: "AU0001".to_string() })),
                    _ => Ok(HttpResponse::InternalServerError().finish()),
                },
            }
        })
        .responder()
}


fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = helpers::db::init_pool(&database_url).unwrap();

    server::new(move || {
            App::with_state(AppState { pool: db_pool.clone() })
                .prefix("/v1")
                .resource("/ping", |r| r.get().f(ping))
                .resource("/register", |r| r.post().f(register))
        })
        .bind("0.0.0.0:8080")
        .expect("Failed to start http server: 0.0.0.0:8080")
        .run();

    println!("Started http server: 0.0.0.0:8080");
}
