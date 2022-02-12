use sha3::{Digest, Sha3_512};

use super::email::UserEmail;
use crate::dtos::users::UserLoginDto;

/// This struct represents a valid user input for authentication.
pub struct UserLoginRequest {
    pub email: UserEmail,
    pub password: String,
}

impl TryFrom<UserLoginDto> for UserLoginRequest {
    type Error = String;

    /// Transforms a [`UserLogin`] payload to a domain-compliant
    /// [`LoginUser`] (valid email address, hashed password).
    fn try_from(value: UserLoginDto) -> Result<Self, Self::Error> {
        let email = UserEmail::parse(value.user.email)?;
        if value.user.password.is_empty() {
            return Err("A password cannot be empty.".to_string());
        }

        let mut hasher = Sha3_512::new();
        hasher.update(value.user.password.as_str());
        let hashed_password = format!("{:x}", hasher.finalize());

        Ok(UserLoginRequest {
            email,
            password: hashed_password,
        })
    }
}
