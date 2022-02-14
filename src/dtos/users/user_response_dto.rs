use serde::Serialize;

/// The User API Response format, as described in the spec, encapsulates
/// user information inside a `user` field.
#[derive(Serialize)]
pub struct UserResponseDto<'a> {
    user: UserResponseFields<'a>,
}

/// The user fields. We accept `bio` and `image` to be [`None`]
/// (translated to `null` in JSON) as they have not a default value on
/// registration.
#[derive(Serialize)]
struct UserResponseFields<'a> {
    username: &'a str,
    email: &'a str,
    bio: Option<&'a str>,
    image: Option<&'a str>,
    token: &'a str,
}

impl<'a> UserResponseDto<'a> {
    /// Constructs a new [`UserResponseDto`] with the given values for the fields
    /// of the [`UserResponseFields`] strcture.
    pub fn new(
        username: &'a str,
        email: &'a str,
        bio: Option<&'a str>,
        image: Option<&'a str>,
        token: &'a str,
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
