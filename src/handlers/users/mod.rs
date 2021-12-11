//! The Users request handlers are responsible for handling CRU* actions on the
//! users:
//! - Create new users with registration (`POST /api/users`) with the
//!   `register` module ;
//! - Read user information (`GET /api/user`): WIP module ;
//! - Update user information (`PUT /api/user`): WIP module ;
//! - Authentication (`POST /api/users/login`): WIP module.

use actix_web::web;

pub mod register;
pub mod user_info;

/// Configure the Users service: registration and authentication.
pub fn config_users(cfg: &mut web::ServiceConfig) {
    cfg.service(register::register);
}

/// Configure the User service: Get user info and Update user.
pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(user_info::user_info);
}
