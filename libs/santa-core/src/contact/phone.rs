use super::ContactMethod;

#[derive(Debug, PartialEq)]
pub struct PhoneNumber {
    value: String,
}

impl PhoneNumber {
    /// Create a new PhoneNumber from the given input
    pub fn new(telno: &str) -> Option<Self> {
        if telno.len() < 6 {
            None
        } else {
            // Add a leading + if none given
            let number = if telno.starts_with('+') {
                telno.to_owned()
            } else {
                String::from("+") + &telno.to_owned()
            };

            //log::info!("New phone number: {}", &number);

            Some(Self { value: number })
        }
    }
}

impl ContactMethod for PhoneNumber {
    /// Get an immutable ref to the value of the telno
    fn value(&self) -> &str {
        &self.value
    }

    /// Send the message via Twilio
    fn send_message(&self, msg: &str) {
        log::info!("{} <-- {}", self.value(), msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_phone_number() {
        let phone = PhoneNumber::new("+441122334455").unwrap();
        assert_eq!(phone.value(), "+441122334455");

        let phone = PhoneNumber::new("441122334455").unwrap();
        assert_eq!(phone.value(), "+441122334455");
    }

    #[test]
    fn new_invalid_phone_number() {
        let phone = PhoneNumber::new("asdf");
        assert_eq!(phone, None);

        let phone = PhoneNumber::new("441122334455").unwrap();
        assert_eq!(phone.value(), "+441122334455");
    }

    #[test]
    fn send_msg() {
        let phone = PhoneNumber::new("+441122334455").unwrap();

        phone.send_message("This is a test!");
    }
}
