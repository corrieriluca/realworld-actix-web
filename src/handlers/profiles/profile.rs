use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetProfileRequest {
    username: String,
}

/// The `GET /api/profiles/:username` endpoint.
/// TODO.
#[get("/{username}")]
async fn get_profile(req: web::Path<GetProfileRequest>) -> HttpResponse {
    HttpResponse::Ok().body(format!("hello {}", req.username))
}
