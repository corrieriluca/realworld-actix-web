//! The Profiles request handlers are responsible for handling CR_D actions on
//! the profiles:
//! - Get an user profile (`GET /api/profiles/:username`) with the `profile`
//!   module ;
//! - Follow an user profile (`POST /api/profiles/:username/follow`) with the
//!   `follow` module ;
//! - Unfollow an user profile (`DELETE /api/profiles/:username/follow`) with
//!   the `unfollow` module ;

use actix_web::web;

pub mod follow;
pub mod profile;
pub mod unfollow;

/// Configure the Profiles service (`/api/profiles/...` endpoints).
pub fn config_profiles(cfg: &mut web::ServiceConfig) {
    cfg.service(profile::get_profile);
    cfg.service(follow::follow_user);
    cfg.service(unfollow::unfollow_user);
}
