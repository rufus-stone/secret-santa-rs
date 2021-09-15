use super::ContactMethod;

#[derive(Debug, PartialEq)]
pub struct Email {
    value: String,
}

impl Email {
    /// Create a new Email from the given input
    pub fn new(email: &str) -> Option<Self> {
        if email.len() < 6 || !email.contains('@') {
            None
        } else {
            Some(Self {
                value: email.to_owned(),
            })
        }
    }
}

impl ContactMethod for Email {
    /// Get an immutable ref to the value of the telno
    fn value(&self) -> &str {
        &self.value
    }

    /// Send the message via email
    fn send_message(&self, msg: &str) {
        log::info!("{} <-- {}", self.value(), msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_email() {
        let email = Email::new("alice@blah.com").unwrap();
        assert_eq!(email.value(), "alice@blah.com");
    }

    #[test]
    fn new_invalid_email() {
        let email = Email::new("a@b.c");
        assert_eq!(email, None);

        let email = Email::new("Not an email");
        assert_eq!(email, None);
    }
}
