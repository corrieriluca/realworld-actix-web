use actix_web::{get, HttpResponse};

use crate::{middlewares, models::users::UserResponse};

#[get("", wrap = "middlewares::Authentication")]
async fn user_info(user: middlewares::AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().json(UserResponse::new(
        &user.user.username,
        &user.user.email,
        user.user.bio.as_deref(),
        user.user.image.as_deref(),
        &user.token,
    ))
}
