//! The Users request handlers are responsible for handling CRU_ actions on the
//! users:
//! - Create new users with registration (`POST /api/users`) with the
//!   `register` module ;
//! - Read user information (`GET /api/user`) with the `user_info` module ;
//! - Update user information (`PUT /api/user`) with the `update` module ;
//! - Authentication (`POST /api/users/login`) with the `login` module.

use actix_web::web;

pub mod login;
pub mod register;
pub mod update;
pub mod user_info;

/// Configure the Users service: registration and authentication.
/// `/api/users/...` endpoints.
pub fn config_users(cfg: &mut web::ServiceConfig) {
    cfg.service(register::register);
    cfg.service(login::login);
}

/// Configure the User service: Get user info and Update user.
/// `/api/user` endpoints.
pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(user_info::user_info);
    cfg.service(update::update);
}
