pub mod email;
pub mod message;
pub mod name;

use email::Email;
use message::Message;
use name::Name;

pub use email::Error as EmailError;
pub use message::Error as MessageError;
pub use name::Error as NameError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Contact {
    pub email: Email,
    pub name: Name,
    pub message: Message,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub email: Option<EmailError>,
    pub name: Option<NameError>,
    pub message: Option<MessageError>,
}

impl Contact {
    pub fn new(email: &str, name: &str, message: &str) -> Result<Self, Error> {
        match (Email::new(email), Name::new(name), Message::new(message)) {
            (Ok(email), Ok(name), Ok(message)) => Ok(Self {
                email,
                name,
                message,
            }),
            (email, name, message) => Err(Error {
                email: email.err(),
                name: name.err(),
                message: message.err(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_all_errors() {
        let contact = Contact::new("", "", "");

        let expected_errors = Error {
            email: Some(EmailError::IsEmpty),
            name: Some(NameError::IsEmpty),
            message: Some(MessageError::IsEmpty),
        };

        assert_eq!(Err(expected_errors), contact);
    }

    #[test]
    fn can_fail_just_email() {
        let contact = Contact::new("", "good", "good");

        let expected_errors = Error {
            email: Some(EmailError::IsEmpty),
            name: None,
            message: None,
        };

        assert_eq!(Err(expected_errors), contact);
    }

    #[test]
    fn can_fail_just_name() {
        let contact = Contact::new("good@foo.com", "", "good");

        let expected_errors = Error {
            email: None,
            name: Some(NameError::IsEmpty),
            message: None,
        };

        assert_eq!(Err(expected_errors), contact);
    }

    #[test]
    fn can_fail_just_message() {
        let contact = Contact::new("good@foo.com", "good", "");

        let expected_errors = Error {
            email: None,
            name: None,
            message: Some(MessageError::IsEmpty),
        };

        assert_eq!(Err(expected_errors), contact);
    }

    #[test]
    fn can_construct_a_contact() {
        let contact = Contact::new("good@foo.com", "joe", "hello world").unwrap();

        assert_eq!("good@foo.com".to_owned(), contact.email.to_string());
        assert_eq!("joe".to_owned(), contact.name.to_string());
        assert_eq!("hello world".to_owned(), contact.message.to_string());
    }
}
