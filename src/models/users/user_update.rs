use serde::Deserialize;

/// The JSON payload model received for a user registration.
#[derive(Deserialize)]
pub struct UserUpdate {
    pub user: UserUpdateFields,
}

#[derive(Deserialize)]
pub struct UserUpdateFields {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}
