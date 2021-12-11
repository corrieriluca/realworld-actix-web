/// Holds a valid username.
/// A valid username meets these criteria:
/// - Must not be empty
/// - Must be at most 60 characters long
/// - Must only contain alphanumeric ASCII characters with underscores
#[derive(Debug)]
pub struct Username(String);

impl Username {
    /// Tries to parse a string into a valid username. Returns [`Err`]
    /// if the [`Username`] criteria are not met.
    pub fn parse(s: String) -> Result<Username, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.chars().count() > 60;
        let is_alphanumeric_with_underscores = s
            .chars()
            .all(|c| char::is_ascii_alphanumeric(&c) || c == '_');

        if is_empty_or_whitespace {
            Err("An username cannot be empty.".to_string())
        } else if is_too_long || !is_alphanumeric_with_underscores {
            Err(format!("{} is not a valid username.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<String> for Username {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};

    use super::Username;

    #[test]
    fn an_empty_username_is_not_valid() {
        assert_err!(Username::parse("".into()));
    }

    #[test]
    fn a_whitespace_only_username_is_not_valid() {
        assert_err!(Username::parse("       ".into()));
    }

    #[test]
    fn a_60_characters_long_username_is_valid() {
        let username = "a".repeat(60);
        assert_ok!(Username::parse(username));
    }

    #[test]
    fn too_long_username_is_not_valid() {
        let username = "a".repeat(61);
        assert_err!(Username::parse(username));
    }

    #[test]
    fn an_username_with_spaces_chars_is_not_valid() {
        assert_err!(Username::parse("a not valid username".into()));
    }

    #[test]
    fn an_username_with_forbidden_chars_is_not_valid() {
        assert_err!(Username::parse("éè!@ç...".into()));
    }

    #[test]
    fn a_valid_username_is_accepted() {
        assert_ok!(Username::parse("a_valid_username_8".into()));
    }
}
