use crate::contact::ContactMethod;

#[derive(Debug)]
pub struct Person<C>
where
    C: ContactMethod,
{
    name: String,
    contact: C,
}

impl<C> Person<C>
where
    C: ContactMethod,
{
    /// Create a new Person genericised over ContactMethod C
    pub fn new(name: &str, contact: C) -> Self {
        Self {
            name: name.to_owned(),
            contact,
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
