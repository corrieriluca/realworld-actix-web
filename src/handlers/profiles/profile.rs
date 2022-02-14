use actix_web::{get, web, HttpResponse};

use crate::middlewares;

/// The `GET /api/profiles/:username` endpoint.
/// TODO.
#[get("/{username}")]
async fn get_profile(
    username: web::Path<String>,
    user: middlewares::MaybeAuthenticatedUser,
) -> HttpResponse {
    // Behave differently if authenticated.
    match user.inner() {
        Some(u) => HttpResponse::Ok().body(format!(
            "Hello {}... you are looking at {username}.",
            u.user.username
        )),
        None => HttpResponse::Ok().body(format!("Hello stranger! You are looking at {username}.")),
    }
}
