use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::models::auth::Claims;

/// Create a Claims struct for a new JWT (valid for one hour)
fn create_claims_from_user<'a>(username: &'a str) -> Claims<'a> {
    let iat = time::OffsetDateTime::now_utc();
    let exp = iat + time::Duration::hours(1);

    Claims::new(
        "conduit-v1",
        &username,
        "conduit-v1",
        exp.unix_timestamp() as u64,
        iat.unix_timestamp() as u64,
    )
}

/// Create a JWT for the specified user (valid for one hour).
pub fn create_jwt_for_user<'a>(
    username: &'a str,
    shared_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = create_claims_from_user(username);
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(shared_secret.as_ref()),
    )
}
