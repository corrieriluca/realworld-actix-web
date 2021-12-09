use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::{auth::create_jwt_for_user, users::NewUser},
    handlers::error::validation_error,
    models::{
        auth::JwtSecret,
        users::{UserRegistration, UserResponse},
    },
    repositories::user_repository::insert_new_user,
};

/// The `POST /api/users` endpoint, used for user registration.
/// Return 201 Created in case of success.
#[post("")]
async fn register(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<JwtSecret>,
    user: web::Json<UserRegistration>,
) -> HttpResponse {
    // Validate the input
    let new_user: NewUser = match user.into_inner().try_into() {
        Ok(new_user) => new_user,
        Err(e) => return validation_error(e.as_ref()),
    };

    // Store the result and respond
    match insert_new_user(&pool, &new_user).await {
        Ok(_) => {
            match create_jwt_for_user(new_user.username.as_ref(), &jwt_secret.into_inner().0) {
                Ok(token) => HttpResponse::Created().json(UserResponse::new(
                    new_user.username.as_ref().into(),
                    new_user.email.as_ref().into(),
                    None,
                    None,
                    token,
                )),
                Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
