//! The Users request handlers are responsible for handling CRU* actions on the
//! users:
//! - Create new users with registration (`POST /api/users`) with the
//!   `register` module ;
//! - Read user information (`GET /api/user`): WIP module ;
//! - Update user information (`PUT /api/user`): WIP module ;
//! - Authentication (`POST /api/users/login`): WIP module.

use actix_web::web;

pub mod register;

/// Configure the Users service: registration and authentication.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register::register);
}
