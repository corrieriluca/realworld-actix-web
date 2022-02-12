//! This is the hendlers module. It gathers all the different request handlers
//! (i.e. endpoints) of the API.

use actix_web::web;

pub mod health_check;
pub mod users;

/// Configure the services for Conduit, not including the `/api`
/// prefix, prepended by the application factory.
/// Called in the application factory.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::health_check);
    cfg.service(web::scope("/users").configure(users::config_users));
    cfg.service(web::scope("/user").configure(users::config_user));
}
