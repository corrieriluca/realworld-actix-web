use serde::{Deserialize, Serialize};

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
