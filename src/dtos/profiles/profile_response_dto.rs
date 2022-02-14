use serde::Serialize;

/// The Profile API Response format, as described in the spec, encapsulates
/// profile information inside a `profile` field.
#[derive(Serialize)]
pub struct ProfileResponseDto<'a> {
    profile: ProfileResponseFields<'a>,
}

/// The profile fields. We accept `bio` and `image` to be [`None`]
/// (translated to `null` in JSON) as they have not a default value on
/// registration. `following` is also an option because it is displayed only
/// when the request is authenticated.
#[derive(Serialize)]
struct ProfileResponseFields<'a> {
    username: &'a str,
    bio: Option<&'a str>,
    image: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    following: Option<bool>,
}

impl<'a> ProfileResponseDto<'a> {
    /// Constructs a new [`ProfileResponseDto`] with the given values for the fields
    /// of the [`ProfileResponseDto`] strcture.
    pub fn new(
        username: &'a str,
        bio: Option<&'a str>,
        image: Option<&'a str>,
        following: Option<bool>,
    ) -> Self {
        Self {
            profile: ProfileResponseFields {
                username,
                bio,
                image,
                following,
            },
        }
    }
}
