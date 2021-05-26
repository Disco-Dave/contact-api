use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Name(String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IsEmpty,
    IsGreaterThan200,
}

impl Name {
    pub fn new(name: &str) -> Result<Self, Error> {
        let name = name.trim();

        if name.is_empty() {
            Err(Error::IsEmpty)
        } else if name.graphemes(true).count() > 200 {
            Err(Error::IsGreaterThan200)
        } else {
            Ok(Self(name.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn does_not_allow_empty_names() {
        assert_eq!(Err(Error::IsEmpty), Name::new(""))
    }

    #[test]
    fn does_not_allow_names_that_are_all_whitespace() {
        assert_eq!(Err(Error::IsEmpty), Name::new("    "))
    }

    #[test]
    fn does_not_allow_names_longer_than_200_characters() {
        let long_name = iter::repeat('a').take(201).collect::<String>();
        assert_eq!(Err(Error::IsGreaterThan200), Name::new(&long_name));
    }

    #[test]
    fn does_allow_names_of_200_characters() {
        let long_name = iter::repeat('a').take(200).collect::<String>();
        assert_eq!(
            Ok(&long_name),
            Name::new(&long_name).map(|n| n.to_string()).as_ref()
        );
    }

    #[test]
    fn trims_names() {
        assert_eq!(
            Ok("scooby doo".to_owned()),
            Name::new("  scooby doo   ").map(|n| n.to_string())
        );
    }

    #[test]
    fn does_not_count_leading_and_trailing_whitespace_as_length() {
        let long_name = iter::repeat('a').take(200).collect::<String>();

        assert_eq!(
            Ok(&long_name),
            Name::new(&format!("  {}   ", &long_name))
                .map(|n| n.to_string())
                .as_ref()
        );
    }
}
