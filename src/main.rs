#![feature(custom_attribute)]

extern crate actix;
extern crate actix_web;
extern crate argon2rs;
extern crate chrono;
#[macro_use] extern crate diesel;
extern crate env_logger;
extern crate futures;
extern crate json;
extern crate nanoid;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate time;
extern crate uuid;

pub mod email;
pub mod helpers;
pub mod models;
pub mod schema;

use actix_web::{server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web::middleware::Logger;
use chrono::Utc;
use diesel::result::Error::DatabaseError;
use diesel::prelude::*;
use email::send_email;
use futures::Future;
use models::user::User;
use schema::users;
use time::Duration;


struct AppState {
    pool: helpers::db::Pool,
}

//
// Request bodies
//
#[derive(Debug, Deserialize)]
struct RegisterReqBody {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct LoginReqBody {
    username: String,
    password: String,
}

//
// Response bodies
//
#[derive(Debug, Serialize)]
struct MeReqBody {
    id: String,
    username: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct LoginResBody {
    access_token: String,
}

#[derive(Debug, Serialize)]
struct ErrorResBody {
    error: String,
}

//
// Controllers
//
/// Pong.
fn ping(_: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

/// Get user information.
fn me(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let authorization = req.headers().get("authorization");
    if authorization.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let conn = req.state().pool.clone().get().unwrap();

    let access_token = authorization.unwrap().to_str().unwrap().split_whitespace().last();
    let query = users::table
        .filter(users::dsl::access_token.eq(&access_token))
        .first::<User>(&*conn)
        .optional();

    match query {
        Ok(Some(user)) => {
            if user.is_activated {
                Ok(HttpResponse::Ok().json(MeReqBody {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                }))
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        },
        Ok(None) => Ok(HttpResponse::Unauthorized().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Login and issue access token.
fn login(req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let conn = req.state().pool.clone().get().unwrap();

    req.json().from_err().and_then(move |body: LoginReqBody| {
        // Get user and verify password.
        let query = users::table
            .filter(users::dsl::username.eq(&body.username))
            .first::<User>(&*conn)
            .optional();

        match query {
            Ok(Some(user)) => {
                if user.verify_password(&body.password) && user.is_activated {
                    if user.access_token.is_some() {
                        Ok(HttpResponse::Ok().json(LoginResBody { access_token: user.access_token.unwrap() }))
                    } else {
                        let new_token = nanoid::generate(64);
                        let update_result = diesel::update(users::table)
                            .set(users::dsl::access_token.eq(&new_token))
                            .execute(&*conn);

                        match update_result {
                            Ok(_) => Ok(HttpResponse::Ok().json(LoginResBody { access_token: new_token })),
                            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
                        }
                    }
                } else {
                    Ok(HttpResponse::Unauthorized().json(ErrorResBody { error: "AU0012".to_string() }))
                }
            },
            Ok(None) => Ok(HttpResponse::Unauthorized().json(ErrorResBody { error: "AU0011".to_string() })),
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        }
    }).responder()
}

/// Register a new user account.
fn register(req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let conn = req.state().pool.clone().get().unwrap();
    req.json().from_err().and_then(move |body: RegisterReqBody| {
        let activate_token = nanoid::generate(16);

        let (hash, salt) = User::make_password_hash(&body.password);
        let user = User {
            id: uuid::Uuid::new_v4().hyphenated().to_string(),
            username: body.username,
            password_hash: hash,
            password_salt: salt,
            email: body.email,
            access_token: None,
            access_token_valid_until: None,
            is_activated: false,
            activate_token: Some(activate_token.to_string()),
            activate_token_valid_until: Some(Utc::now().naive_utc() + Duration::hours(2)),
        };
        
        let insert_result = diesel::insert_into(users::table)
            .values(&user)
            .execute(&*conn);

        match insert_result {
            Ok(_) => {
                send_email(&user.email, &"인증 메일입니다.".to_string(), &activate_token);
                Ok(HttpResponse::Ok().body("Ok"))
            },
            Err(e) => match e {
                DatabaseError(_, _) => Ok(HttpResponse::BadRequest().json(ErrorResBody { error: "AU0001".to_string() })),
                _ => Ok(HttpResponse::InternalServerError().finish()),
            },
        }
    }).responder()
}


fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = helpers::db::init_pool(&database_url).unwrap();

    server::new(move || {
            App::with_state(AppState { pool: db_pool.clone() })
                .middleware(Logger::default())
                .prefix("/v1")
                .resource("/ping", |r| r.get().f(ping))
                .resource("/me", |r| r.get().f(me))
                .resource("/login", |r| r.post().f(login))
                .resource("/register", |r| r.post().f(register))
        })
        .bind("0.0.0.0:8080")
        .expect("Failed to start http server: 0.0.0.0:8080")
        .run();
}
