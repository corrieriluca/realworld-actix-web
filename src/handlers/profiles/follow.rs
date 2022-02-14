use std::ops::Deref;

use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::error::{validation_error, ErrorResponse},
    dtos::profiles::profile_response_dto::ProfileResponseDto,
    middlewares,
    repositories::{followers_repository::follow, user_repository::get_user_by_username},
};

#[post("/{username}/follow")]
async fn follow_user(
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
        return validation_error("Cannot follow yourself!");
    }

    match follow(&pool, &user.user.username, &username).await {
        Ok(_) => HttpResponse::Ok().json(ProfileResponseDto::new(
            &username,
            profile.bio.as_deref(),
            profile.image.as_deref(),
            Some(true),
        )),
        Err(e) => match e {
            sqlx::Error::Database(_) => {
                validation_error("Unable to follow. You might already follow this user.")
            },
            _ => HttpResponse::InternalServerError().body("Unexpected error happened."),
        },
    }
}
