use crate::{contact::ContactMethod, person::Person};

/// A Gift is just a tuple of references to two Person structs
pub struct Gift<'a, C: ContactMethod>(&'a Person<C>, &'a Person<C>);

impl<C: ContactMethod> Gift<'_, C> {
    /// Get an immutable ref to the Gift giver
    pub fn giver(&self) -> &Person<C> {
        self.0
    }

    /// Get an immutable ref to the Gift recipient
    pub fn recipient(&self) -> &Person<C> {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contact::phone::PhoneNumber;

    #[test]
    fn new_gift() {
        let gifter = Person::new("Alice", PhoneNumber::new("441122334455").unwrap());
        let giftee = Person::new("Bob", PhoneNumber::new("+442233445511").unwrap());

        let gift = Gift(&gifter, &giftee);

        assert_eq!(
            gift.giver(),
            &Person::new("Alice", PhoneNumber::new("441122334455").unwrap())
        );
        assert_eq!(
            gift.recipient(),
            &Person::new("Bob", PhoneNumber::new("+442233445511").unwrap())
        );
    }
}
