use std::ops::Deref;

use actix_web::{delete, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::error::{validation_error, ErrorResponse},
    dtos::profiles::profile_response_dto::ProfileResponseDto,
    middlewares,
    repositories::{followers_repository::unfollow, user_repository::get_user_by_username},
};

/// The `DELETE /api/profiles/:username/follow` endpoint.
/// Returns 200 with the unfollowed profile upon success.
/// Returns 404 if the user to follow is not found.
/// Returns 422 in other cases (self-unfollowing).
/// Unfollowing an user you're not following does not trigger an error.
#[delete("/{username}/follow")]
async fn unfollow_user(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    user: middlewares::AuthenticatedUser,
) -> HttpResponse {
    // Retrieve profile
    let profile = match get_user_by_username(&pool, &username).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().json(ErrorResponse::new("User not found.")),
    };

    // Check the users are different
    if username.deref() == &user.user.username {
        return validation_error("Cannot unfollow yourself!");
    }

    match unfollow(&pool, &user.user.username, &username).await {
        Ok(_) => HttpResponse::Ok().json(ProfileResponseDto::new(
            &username,
            profile.bio.as_deref(),
            profile.image.as_deref(),
            Some(false),
        )),
        Err(e) => match e {
            sqlx::Error::Database(_) => {
                validation_error("Unable to unfollow. You may not already be following this user.")
            },
            _ => HttpResponse::InternalServerError().body("Unexpected error happened."),
        },
    }
}
