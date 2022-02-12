use serde::Deserialize;

/// The JSON payload model received for a user registration.
#[derive(Deserialize)]
pub struct UserRegistrationDto {
    pub user: UserRegistrationFields,
}

#[derive(Deserialize)]
pub struct UserRegistrationFields {
    pub username: String,
    pub email: String,
    pub password: String,
}
