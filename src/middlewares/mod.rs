//! This module contains all the custom middlewares for the application.

pub mod auth;

pub use auth::{AuthenticatedUser, AuthenticationMiddlewareFactory, MaybeAuthenticatedUser};
