use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::models::auth::Claims;

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
