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
pub mod templates;

use actix_web::{server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web::middleware::{Logger};
use actix_web::middleware::cors::Cors;
use chrono::Utc;
use diesel::result::Error::DatabaseError;
use diesel::prelude::*;
use email::send_email;
use futures::Future;
use helpers::api::*;
use helpers::token::make_firebase_token;
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
    redirection_url: String,
}

#[derive(Debug, Deserialize)]
struct LoginReqBody {
    username: String,
    password: String,
    services: Option<Vec<String>>,
}

//
// Response bodies
//
#[derive(Debug, Serialize)]
struct MeResBody {
    id: String,
    username: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct LoginResBody {
    access_token: String,
    firebase_token: Option<String>,
}


//
// Controllers
//
/// Pong.
fn ping(_: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

/// Activate the account.
fn activate(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let token: String;
    match req.match_info().query("token") {
        Ok(t) => token = t,
        Err(_) => {
            return Ok(bad_request(""));
        }
    };

    let conn = req.state().pool.clone().get().unwrap();
    let query = users::table
        .filter(users::dsl::activation_token.eq(token))
        .first::<User>(&*conn)
        .optional();

    match query {
        Ok(Some(user)) => {
            if user.activation_token_valid_until.unwrap() < Utc::now().naive_utc() {
                Ok(bad_request(""))
            } else {
                // Activate the account and return redirection url.
                let update_result = diesel::update(users::table)
                    .set(users::dsl::is_activated.eq(&true))
                    .execute(&*conn);
                
                match update_result {
                    Ok(_) => Ok(
                        HttpResponse::Found()
                            .header("Location", user.activation_redirection_url.unwrap())
                            .finish()
                    ),
                    Err(_) => Ok(internal_server_error()),
                }
            }
        },
        Ok(None) => Ok(bad_request("")),
        Err(_) => Ok(internal_server_error()),
    }
}

/// Get user information.
fn me(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let access_token: &str;
    match req.headers().get("authorization") {
        Some(authorization) => {
            if authorization != "" {
                access_token = authorization.to_str().unwrap().split_whitespace().last().unwrap();
            } else {
                return Ok(unauthorized(""));
            }
        },
        _ => return Ok(unauthorized("")),
    };

    let conn = req.state().pool.clone().get().unwrap();
    let query = users::table
        .filter(users::dsl::access_token.eq(&access_token))
        .first::<User>(&*conn)
        .optional();

    match query {
        Ok(Some(user)) => {
            if user.is_activated {
                Ok(HttpResponse::Ok().json(MeResBody {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                }))
            } else {
                Ok(unauthorized(""))
            }
        },
        Ok(None) => Ok(unauthorized("")),
        Err(_) => Ok(internal_server_error()),
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
                    let firebase_token;
                    if body.services.is_some() && body.services.unwrap().contains(&"firebase".to_string()) {
                        firebase_token = Some(make_firebase_token(user.id));
                    } else {
                        firebase_token = None;
                    }

                    if user.access_token.is_some() {
                        Ok(HttpResponse::Ok().json(LoginResBody { 
                            access_token: user.access_token.unwrap(),
                            firebase_token: firebase_token,
                        }))
                    } else {
                        let new_token = nanoid::generate(64);
                        let update_result = diesel::update(users::table)
                            .set(users::dsl::access_token.eq(&new_token))
                            .execute(&*conn);

                        match update_result {
                            Ok(_) => Ok(HttpResponse::Ok().json(LoginResBody { 
                                access_token: new_token,
                                firebase_token: firebase_token,
                            })),
                            Err(_) => Ok(internal_server_error()),
                        }
                    }
                } else {
                    Ok(unauthorized("AU0012"))
                }
            },
            Ok(None) => Ok(unauthorized("AU0011")),
            Err(_) => Ok(internal_server_error()),
        }
    }).responder()
}

/// Register a new user account.
fn register(req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let conn = req.state().pool.clone().get().unwrap();
    req.json().from_err().and_then(move |body: RegisterReqBody| {
        let activation_token = nanoid::generate(16);

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
            activation_token: Some(activation_token.to_string()),
            activation_token_valid_until: Some(Utc::now().naive_utc() + Duration::hours(2)),
            activation_redirection_url: Some(body.redirection_url),
        };
        
        let insert_result = diesel::insert_into(users::table)
            .values(&user)
            .execute(&*conn);

        match insert_result {
            Ok(_) => {
                let title = "LYnLab 계정 인증 메일입니다.".to_string();
                let body = templates::email_body_activation(&activation_token);
                send_email(&user.email, &title, &body);

                Ok(HttpResponse::Ok().body("Ok"))
            },
            Err(e) => match e {
                DatabaseError(_, _) => Ok(bad_request("AU0001")),
                _ => Ok(internal_server_error()),
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
                .configure(|app| Cors::for_app(app)
                    .send_wildcard()
                    .resource("/ping", |r| r.get().f(ping))
                    .resource(r"/activate/{token}", |r| r.get().f(activate))
                    .resource("/me", |r| r.get().f(me))
                    .resource("/login", |r| r.post().f(login))
                    .resource("/register", |r| r.post().f(register))
                    .register()
                )
                .prefix("/v1")
        })
        .bind("0.0.0.0:8080")
        .expect("Failed to start http server: 0.0.0.0:8080")
        .run();
}
