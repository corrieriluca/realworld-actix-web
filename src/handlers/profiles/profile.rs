use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::error::ErrorResponse,
    dtos::profiles::profile_response_dto::ProfileResponseDto,
    middlewares,
    repositories::{followers_repository::is_following, user_repository::get_user_by_username},
};

/// The `GET /api/profiles/:username` endpoint.
/// Returns 200 with the profile if the user is found (the presence of the
/// `following` field depends on authentication).
/// Returns 404 if the user is not found.
#[get("/{username}")]
async fn get_profile(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    user: middlewares::MaybeAuthenticatedUser,
) -> HttpResponse {
    // Retrieve profile
    let profile = match get_user_by_username(&pool, &username).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().json(ErrorResponse::new("User not found.")),
    };

    // Behave differently if authenticated.
    match user.inner() {
        // Not authenticated
        None => HttpResponse::Ok().json(ProfileResponseDto::new(
            &profile.username,
            profile.bio.as_deref(),
            profile.image.as_deref(),
            None,
        )),
        // Authenticated, check if following
        Some(u) => match is_following(&pool, &u.user.username, &username).await {
            Ok(following) => HttpResponse::Ok().json(ProfileResponseDto::new(
                &profile.username,
                profile.bio.as_deref(),
                profile.image.as_deref(),
                Some(following),
            )),
            Err(_) => HttpResponse::InternalServerError().body("Unexpected error happened."),
        },
    }
}
