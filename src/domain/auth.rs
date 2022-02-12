//! This module contains several strutures and functions dealing with
//! authentication with a JSON Web Token (JWT).

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Just a wrapper around a string that holds a JWT shared secret.
pub struct JwtSecret(pub String);

/// The structure defining the payload part of a JWT.
#[derive(Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
    iat: u64,
}

impl Claims {
    /// Creates a new Claims structure with the given values as fields.
    pub fn new(iss: String, sub: String, aud: String, exp: u64, iat: u64) -> Self {
        Self {
            iss,
            sub,
            aud,
            exp,
            iat,
        }
    }

    /// Get a reference to the username referenced in this claims (`sub` field).
    pub fn username(&self) -> &str {
        self.sub.as_ref()
    }
}

/// Create a Claims struct for a new JWT (valid for one hour)
fn create_claims_from_user(username: &str) -> Claims {
    let iat = time::OffsetDateTime::now_utc();
    let exp = iat + time::Duration::hours(1);

    Claims::new(
        "conduit-v1".to_owned(),
        username.to_owned(),
        "conduit-v1".to_owned(),
        exp.unix_timestamp() as u64,
        iat.unix_timestamp() as u64,
    )
}

/// Create a JWT for the specified user (valid for one hour).
pub fn create_jwt_for_user(
    username: &str,
    shared_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = create_claims_from_user(username);
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(shared_secret.as_ref()),
    )
}

/// Decode a JWT token into a [`Claims`] struct with the given shared secret.
pub fn decode_token(
    token: &str,
    shared_secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(shared_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
