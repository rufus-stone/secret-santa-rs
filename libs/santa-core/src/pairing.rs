use crate::{contact::ContactMethod, person::Person};

/// A Pairing is just a tuple of references to two Person structs, where .0 is the giver and .1 is the recipient
pub struct Pairing<'a, C: ContactMethod>(&'a Person<'a, C>, &'a Person<'a, C>);

impl<'a, C: ContactMethod> Pairing<'a, C> {
    pub fn new(giver: &'a Person<C>, recipient: &'a Person<C>) -> Self {
        Self(giver, recipient)
    }
}

impl<C: ContactMethod> Pairing<'_, C> {
    /// Get an immutable ref to the gift giver
    pub fn giver(&self) -> &Person<C> {
        self.0
    }

    /// Get an immutable ref to the gift recipient
    pub fn recipient(&self) -> &Person<C> {
        self.1
    }
}

impl<C: ContactMethod> std::fmt::Debug for Pairing<'_, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Pairing{{{:?} -> {:?}}}", self.giver(), self.recipient())
    }
}

/// A GiftPairing is just a tuple of two Person structs, where .0 is the giver and .1 is the recipient
pub struct GiftPairing<'a, C: ContactMethod>(Person<'a, C>, Person<'a, C>);

impl<'a, C: ContactMethod> GiftPairing<'a, C> {
    pub fn new(giver: Person<'a, C>, recipient: Person<'a, C>) -> Self {
        Self(giver, recipient)
    }

    /// Get an immutable ref to the gift giver
    pub fn giver(&self) -> &Person<C> {
        &self.0
    }

    /// Get an immutable ref to the gift recipient
    pub fn recipient(&self) -> &Person<C> {
        &self.1
    }
}

impl<'a, C: ContactMethod> std::fmt::Debug for GiftPairing<'a, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "GiftPairing{{{:?} -> {:?}}}",
            self.giver(),
            self.recipient()
        )
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

        let pairing = Pairing(&gifter, &giftee);

        assert_eq!(
            pairing.giver(),
            &Person::new("Alice", PhoneNumber::new("441122334455").unwrap())
        );
        assert_eq!(
            pairing.recipient(),
            &Person::new("Bob", PhoneNumber::new("+442233445511").unwrap())
        );
    }
}
