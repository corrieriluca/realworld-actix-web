use serde::Serialize;

/// The User API Response format, as described in the spec, encapsulates
/// user information inside a `user` field.
#[derive(Serialize)]
pub struct UserResponse {
    user: UserResponseFields,
}

/// The user fields. We accept `bio` and `image` to be [`None`]
/// (translated to `null` in JSON) as they have not a default value on
/// registration.
#[derive(Serialize)]
struct UserResponseFields {
    username: String,
    email: String,
    bio: Option<String>,
    image: Option<String>,
    token: String,
}

impl UserResponse {
    pub fn new(
        username: String,
        email: String,
        bio: Option<String>,
        image: Option<String>,
        token: String,
    ) -> Self {
        Self {
            user: UserResponseFields {
                username,
                email,
                bio,
                image,
                token,
            },
        }
    }
}
