use serde::Deserialize;

/// The JSON payload model received for a user login.
#[derive(Deserialize)]
pub struct UserLogin {
    pub user: UserLoginFields,
}

#[derive(Deserialize)]
pub struct UserLoginFields {
    pub email: String,
    pub password: String,
}
