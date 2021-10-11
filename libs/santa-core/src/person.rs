use std::marker::PhantomData;

use crate::contact::ContactMethod;

#[derive(PartialEq, Clone)]
pub struct Person<'a, C>
where
    C: ContactMethod,
{
    name: String,
    contact: C,
    phantom: PhantomData<&'a C>,
}

impl<'a, C> Person<'a, C>
where
    C: ContactMethod,
{
    /// Create a new Person genericised over ContactMethod C
    pub fn new(name: &str, contact: C) -> Self {
        Self {
            name: name.to_owned(),
            contact,
            phantom: PhantomData,
        }
    }

    /// Get an immutable ref to the name of the Person
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get an immutable ref to the contact method of the Person
    pub fn contact(&self) -> &C {
        &self.contact
    }
}

impl<C: ContactMethod> std::fmt::Debug for Person<'_, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Person{{{} ({})}}", self.name(), self.contact().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contact::phone::PhoneNumber;

    #[test]
    fn new_person() {
        let person = Person::new("Alice", PhoneNumber::new("441122334455").unwrap());

        assert_eq!(person.name(), "Alice");

        assert_eq!(person.contact(), &PhoneNumber::new("441122334455").unwrap());
        assert_eq!(person.contact().value(), "+441122334455");
    }
}
