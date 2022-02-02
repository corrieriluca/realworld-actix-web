use actix_web::{put, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::{auth::create_jwt_for_user, users::UpdateUser},
    handlers::error::validation_error,
    middlewares,
    models::{
        auth::JwtSecret,
        users::{UserResponse, UserUpdate},
    },
    repositories::user_repository::{get_user_by_username, update_user},
};

/// The `PUT /api/user` endpoint. **Requires authentication.**
/// Return 200 OK with an user response as JSON body.
/// Return 401 Unauthorized (by the authentication middleware) if there is not
/// a valid authentication.
#[put("", wrap = "middlewares::Authentication")]
async fn update(
    user: middlewares::AuthenticatedUser,
    jwt_secret: web::Data<JwtSecret>,
    pool: web::Data<PgPool>,
    update: web::Json<UserUpdate>,
) -> HttpResponse {
    // Validate the input
    let updated_user: UpdateUser = match update.into_inner().try_into() {
        Ok(updated_user) => updated_user,
        Err(e) => return validation_error(e.as_ref()),
    };

    if updated_user.is_all_none() {
        return validation_error("No update provided!");
    }

    // Update in the database and respond
    match update_user(&pool, &user.user.username, &updated_user).await {
        Ok(new_username) => {
            // Get the new user
            match get_user_by_username(&pool, new_username.as_ref()).await {
                Ok(user) => {
                    // Generate token and respond
                    match create_jwt_for_user(user.username.as_ref(), &jwt_secret.into_inner().0) {
                        Ok(token) => HttpResponse::Ok().json(UserResponse::new(
                            user.username.as_ref(),
                            user.email.as_ref(),
                            user.bio.as_deref(),
                            user.image.as_deref(),
                            &token,
                        )),
                        Err(_) => {
                            HttpResponse::InternalServerError().body("Unexpected error happened.")
                        },
                    }
                },
                Err(_) => HttpResponse::InternalServerError().body("Unexpected error happened."),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Unexpected error happened."),
    }
}
