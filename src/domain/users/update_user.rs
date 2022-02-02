use sha3::{Digest, Sha3_512};

use super::{email::UserEmail, username::Username};
use crate::models::users::UserUpdate;

/// This struct represents a valid user input for registration.
pub struct UpdateUser {
    pub username: Option<Username>,
    pub email: Option<UserEmail>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl UpdateUser {
    /// Returns true if all the fields of the struct are [`None`] variant.
    pub fn is_all_none(&self) -> bool {
        self.username.is_none()
            && self.email.is_none()
            && self.password.is_none()
            && self.bio.is_none()
            && self.image.is_none()
    }
}

impl TryFrom<UserUpdate> for UpdateUser {
    type Error = String;

    /// Transforms a [`UserUpdate`] payload to a domain-compliant
    /// [`UpdateUser`] (valid username, valid email address, hashed password,
    /// valid bio and image).
    fn try_from(value: UserUpdate) -> Result<Self, Self::Error> {
        let mut username = None;
        if let Some(uname) = value.user.username {
            username = Some(Username::parse(uname)?);
        }

        let mut email = None;
        if let Some(user_email) = value.user.email {
            email = Some(UserEmail::parse(user_email)?);
        }

        let mut password = None;
        if let Some(user_password) = value.user.password {
            if user_password.is_empty() {
                return Err("A password cannot be empty".into());
            }
            let mut hasher = Sha3_512::new();
            hasher.update(user_password);
            password = Some(format!("{:x}", hasher.finalize()));
        }

        let mut bio = None;
        if let Some(user_bio) = value.user.bio {
            if user_bio.chars().count() > 140 {
                return Err("The bio is too long! (140 chars max.)".into());
            }
            bio = Some(user_bio);
        }

        let mut image = None;
        if let Some(image_uri) = value.user.image {
            if validator::validate_url(&image_uri) {
                image = Some(image_uri);
            } else {
                return Err(format!("{image_uri} is not a valid URI!"));
            }
        }

        Ok(UpdateUser {
            username,
            email,
            password,
            bio,
            image,
        })
    }
}
