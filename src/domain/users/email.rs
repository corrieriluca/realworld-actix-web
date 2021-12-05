/// Holds a valid user email address.
#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    /// Tries to parse a string into a valid email address. Returns [`Err`] if
    /// the input is not a valid email address.
    pub fn parse(s: String) -> Result<UserEmail, String> {
        if validator::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email.", s))
        }
    }
}

impl AsRef<String> for UserEmail {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use fake::{faker::internet::en::SafeEmail, Fake};

    use super::UserEmail;

    #[test]
    fn empty_string_is_not_valid() {
        assert_err!(UserEmail::parse("".into()));
    }

    #[test]
    fn missing_domain_is_not_valid() {
        assert_err!(UserEmail::parse("username".into()));
    }

    #[test]
    fn missing_username_is_not_valid() {
        assert_err!(UserEmail::parse("@domain.com".into()));
    }

    #[test]
    fn random_valid_email_is_valid() {
        for _ in 0..100 {
            let email = SafeEmail().fake();
            assert_ok!(UserEmail::parse(email));
        }
    }
}
