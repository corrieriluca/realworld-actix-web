use actix_web::{get, HttpResponse};

use crate::{dtos::users::UserResponseDto, middlewares};

/// The `GET /api/user` endpoint. **Requires authentication.**
/// Return 200 OK with an user response as JSON body.
/// Return 401 Unauthorized (by the authentication middleware) if there is not
/// a valid authentication.
#[get("", wrap = "middlewares::Authentication")]
async fn user_info(user: middlewares::AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().json(UserResponseDto::new(
        &user.user.username,
        &user.user.email,
        user.user.bio.as_deref(),
        user.user.image.as_deref(),
        &user.token,
    ))
}
