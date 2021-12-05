use actix_web::{post, web, Either, HttpResponse};

use crate::{
    domain::users::new_user::NewUser,
    models::users::{UserRegistration, UserResponse},
};

type RegisterResult = Either<web::Json<UserResponse>, HttpResponse>;

/// The `POST /api/users` endpoint, used for user registration.
#[post("")]
async fn register(user: web::Json<UserRegistration>) -> RegisterResult {
    let new_user: NewUser = match user.into_inner().try_into() {
        Ok(new_user) => new_user,
        Err(e) => return Either::Right(HttpResponse::BadRequest().body(e)),
    };

    Either::Left(web::Json(UserResponse::new(
        new_user.username.as_ref().to_string(),
        new_user.email.as_ref().to_string(),
        None,
        None,
        "token".into(),
    )))
}
