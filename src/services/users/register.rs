use actix_web::{post, web};
use serde::Deserialize;

use crate::models::users::UserResponse;

#[derive(Deserialize)]
struct UserRegistration {
    user: UserRegistrationFields,
}

#[derive(Deserialize)]
struct UserRegistrationFields {
    username: String,
    email: String,
    password: String,
}

/// The `POST /api/users` endpoint, used for user registration.
#[post("")]
async fn register(user: web::Json<UserRegistration>) -> web::Json<UserResponse> {
    web::Json(UserResponse::new(
        user.user.username.to_owned(),
        user.user.email.to_owned(),
        None,
        None,
        "token".into(),
    ))
}
