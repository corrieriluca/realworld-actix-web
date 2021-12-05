use actix_web::{post, web, Either, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::users::new_user::NewUser,
    models::users::{UserRegistration, UserResponse},
    repositories::user_repository::insert_new_user,
};

type RegisterResult = Either<web::Json<UserResponse>, HttpResponse>;

/// The `POST /api/users` endpoint, used for user registration.
#[post("")]
async fn register(pool: web::Data<PgPool>, user: web::Json<UserRegistration>) -> RegisterResult {
    // Validate the input
    let new_user: NewUser = match user.into_inner().try_into() {
        Ok(new_user) => new_user,
        Err(e) => return Either::Right(HttpResponse::BadRequest().body(e)),
    };

    // Store the result and respond
    match insert_new_user(&pool, &new_user).await {
        Ok(_) => Either::Left(web::Json(UserResponse::new(
            new_user.username.as_ref().into(),
            new_user.email.as_ref().into(),
            None,
            None,
            "token".into(),
        ))),
        Err(e) => Either::Right(HttpResponse::InternalServerError().body(format!("{}", e))),
    }
}
