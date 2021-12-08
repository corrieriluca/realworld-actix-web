//! This module contains several strutures dealing with authentication
//! with a JSON Web Token (JWT).

use serde::{Deserialize, Serialize};

/// Just a wrapper around a string that holds a JWT shared secret.
pub struct JwtSecret(pub String);

/// The structure defining the payload part of a JWT.
#[derive(Serialize, Deserialize)]
pub struct Claims<'a> {
    iss: &'static str,
    sub: &'a str,
    aud: &'static str,
    exp: u64,
    iat: u64,
}

impl<'a> Claims<'a> {
    /// Creates a new Claims structure with the given values as fields.
    pub fn new(iss: &'static str, sub: &'a str, aud: &'static str, exp: u64, iat: u64) -> Self {
        Self {
            iss,
            sub,
            aud,
            exp,
            iat,
        }
    }
}
