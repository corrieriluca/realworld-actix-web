use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::{auth::create_jwt_for_user, error::validation_error, users::LoginUser},
    models::{
        auth::JwtSecret,
        users::{UserLogin, UserResponse},
    },
    repositories::user_repository::get_user_with_password_by_email,
};

/// The `POST /api/users/login` endpoint used for authentication.
/// Return 200 OK in case of success.
#[post("login")]
async fn login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<JwtSecret>,
    user: web::Json<UserLogin>,
) -> HttpResponse {
    // Validate the input
    let login_user: LoginUser = match user.into_inner().try_into() {
        Ok(login_user) => login_user,
        Err(e) => return validation_error(e.as_ref()),
    };

    // Get the user with its password
    match get_user_with_password_by_email(&pool, login_user.email.as_ref()).await {
        Ok(user) => {
            // Try to match the passwords
            if user.password == login_user.password {
                // Return a JWT token if success
                match create_jwt_for_user(&user.username, &jwt_secret.into_inner().0) {
                    Ok(token) => HttpResponse::Ok().json(UserResponse::new(
                        &user.username,
                        &user.email,
                        user.bio.as_deref(),
                        user.image.as_deref(),
                        &token,
                    )),
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Unexpected error happened.")
                    },
                }
            } else {
                HttpResponse::Forbidden().body("Incorrect email or password.")
            }
        },
        Err(_) => HttpResponse::Forbidden().body("Incorrect email or password."),
    }
}
