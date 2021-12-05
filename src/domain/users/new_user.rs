use sha3::{Digest, Sha3_512};

use super::{email::UserEmail, username::Username};
use crate::models::users::UserRegistration;

pub struct NewUser {
    pub username: Username,
    pub email: UserEmail,
    pub password: String,
}

impl TryFrom<UserRegistration> for NewUser {
    type Error = String;

    /// Transforms a [`UserRegistration`] payload to a domain-compliant
    /// [`NewUser`] (valid username, valid email address, hashed password).
    fn try_from(value: UserRegistration) -> Result<Self, Self::Error> {
        let username = Username::parse(value.user.username)?;
        let email = UserEmail::parse(value.user.email)?;
        let mut hasher = Sha3_512::new();
        hasher.update(value.user.password.as_str());
        let hashed_password = format!("{:x}", hasher.finalize());

        Ok(NewUser {
            username,
            email,
            password: hashed_password,
        })
    }
}
