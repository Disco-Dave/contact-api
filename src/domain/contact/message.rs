use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Message(String);

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for Message {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IsEmpty,
    IsGreaterThan2000,
}

impl Message {
    pub fn new(message: &str) -> Result<Self, Error> {
        let message = message.trim();

        if message.is_empty() {
            Err(Error::IsEmpty)
        } else if message.graphemes(true).count() > 2000 {
            Err(Error::IsGreaterThan2000)
        } else {
            Ok(Self(message.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn does_not_allow_empty_messages() {
        assert_eq!(Err(Error::IsEmpty), Message::new(""))
    }

    #[test]
    fn does_not_allow_messages_that_are_all_whitespace() {
        assert_eq!(Err(Error::IsEmpty), Message::new("    "))
    }

    #[test]
    fn does_not_allow_messages_longer_than_2000_characters() {
        let long_message = iter::repeat('a').take(2001).collect::<String>();
        assert_eq!(Err(Error::IsGreaterThan2000), Message::new(&long_message));
    }

    #[test]
    fn does_allow_messages_of_200_characters() {
        let long_message = iter::repeat('a').take(2000).collect::<String>();
        assert_eq!(
            Ok(&long_message),
            Message::new(&long_message).map(|n| n.to_string()).as_ref()
        );
    }

    #[test]
    fn trims_messages() {
        assert_eq!(
            Ok("scooby doo".to_owned()),
            Message::new("  scooby doo   ").map(|n| n.to_string())
        );
    }

    #[test]
    fn does_not_count_leading_and_trailing_whitespace_as_length() {
        let long_message = iter::repeat('a').take(2000).collect::<String>();

        assert_eq!(
            Ok(&long_message),
            Message::new(&format!("  {}   ", &long_message))
                .map(|n| n.to_string())
                .as_ref()
        );
    }
}
