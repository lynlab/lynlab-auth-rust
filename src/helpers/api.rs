use actix_web::HttpResponse;

#[derive(Debug, Serialize)]
struct ErrorResBody {
    error: String,
}


/// 400 Bad Request
pub fn bad_request(mut error: &str) -> HttpResponse {
    if error.len() == 0 {
        error = "AU0000"
    }
    HttpResponse::BadRequest().json(ErrorResBody { error: error.to_string() })
}

/// 401 Unauthorized
pub fn unauthorized(mut error: &str) -> HttpResponse {
    if error.len() == 0 {
        error = "AU0000"
    }
    HttpResponse::Unauthorized().json(ErrorResBody { error: error.to_string() })
}

/// 500 Internal Server Error
pub fn internal_server_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}
