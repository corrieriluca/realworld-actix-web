use actix_web::{post, web, Either, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::auth::create_jwt_for_user,
    models::users::{UserRegistration, UserResponse},
    repositories::user_repository::insert_new_user,
    startup::JwtSecret,
};

/// The type returned by the register request handler, either a UserResponse
/// encoded as JSON, or a HttpResponse (in case of error).
type RegisterResult = Either<web::Json<UserResponse>, HttpResponse>;

/// The `POST /api/users` endpoint, used for user registration.
#[post("")]
async fn register(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<JwtSecret>,
    user: web::Json<UserRegistration>,
) -> RegisterResult {
    // Validate the input
    let new_user = match user.into_inner().try_into() {
        Ok(new_user) => new_user,
        Err(e) => return Either::Right(HttpResponse::BadRequest().body(e)),
    };

    // Store the result and respond
    match insert_new_user(&pool, &new_user).await {
        Ok(_) => {
            match create_jwt_for_user(new_user.username.as_ref(), &jwt_secret.into_inner().0) {
                Ok(token) => Either::Left(web::Json(UserResponse::new(
                    new_user.username.as_ref().into(),
                    new_user.email.as_ref().into(),
                    None,
                    None,
                    token,
                ))),
                Err(e) => Either::Right(HttpResponse::InternalServerError().body(format!("{}", e))),
            }
        },
        Err(e) => Either::Right(HttpResponse::InternalServerError().body(format!("{}", e))),
    }
}
