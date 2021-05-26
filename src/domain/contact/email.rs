use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Email(String);

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IsEmpty,
    IsGreaterThan300,
    IsMissingAtSign,
}

impl Email {
    pub fn new(email: &str) -> Result<Self, Error> {
        let email = email.trim();

        if email.is_empty() {
            Err(Error::IsEmpty)
        } else if email.graphemes(true).count() > 300 {
            Err(Error::IsGreaterThan300)
        } else if !email.contains(|c| c == '@') {
            Err(Error::IsMissingAtSign)
        } else {
            Ok(Self(email.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn does_not_allow_empty_email() {
        assert_eq!(Err(Error::IsEmpty), Email::new(""));
    }

    #[test]
    fn does_not_allow_all_whitespace_for_email() {
        assert_eq!(Err(Error::IsEmpty), Email::new("      "));
    }

    #[test]
    fn does_not_allow_all_more_than_300_characters_for_email() {
        let long_message = iter::repeat('@').take(301).collect::<String>();
        assert_eq!(Err(Error::IsGreaterThan300), Email::new(&long_message));
    }

    #[test]
    fn does_not_allow_email_to_not_have_at_sign() {
        assert_eq!(
            Err(Error::IsMissingAtSign),
            Email::new("someemail_at_domain")
        );
    }

    #[test]
    fn accepts_valid_emails() {
        let valid_emails = vec![
            "email@example.com",
            "firstname.lastname@example.com",
            "email@subdomain.example.com",
            "firstname+lastname@example.com",
            "email@123.123.123.123",
            "email@[123.123.123.123]",
            "1234567890@example.com",
            "email@example-one.com",
            "_______@example.com",
            "email@example.name",
            "email@example.museum",
            "email@example.co.jp",
            "firstname-lastname@example.com",
            "“email”@example.com",
        ];

        for email in valid_emails {
            assert_eq!(
                Ok(email),
                Email::new(email)
                    .map(|e| e.to_string())
                    .as_ref()
                    .map(|s| s.as_ref())
            );
        }
    }
}
