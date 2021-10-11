use std::marker::PhantomData;

use crate::pairing::{GiftPairing, Pairing};
use crate::person::Person;
use crate::{algorithm::Algorithm, contact::ContactMethod};

pub type SantaResult<T> = std::result::Result<T, String>;

#[derive(Debug)]
pub struct Santa<'a, C, A>
where
    C: ContactMethod,
    A: Algorithm<'a, C>,
{
    participants: Vec<Person<'a, C>>,
    algorithm: A,
    phantom: PhantomData<&'a C>,
}

impl<'a, C, A> Santa<'a, C, A>
where
    C: ContactMethod,
    A: Algorithm<'a, C>,
{
    /// Create a new Santa from the specified participants
    pub fn new(participants: Vec<Person<'a, C>>, algorithm: A) -> SantaResult<Self> {
        // Need at least 3 people to participate!
        if participants.len() < 3 {
            return Err(String::from(
                "Need at least 3 participants for Secret Santa!",
            ));
        }

        log::info!(
            "Creating new Santa with {} participants",
            participants.len()
        );

        Ok(Self {
            participants,
            algorithm,
            phantom: PhantomData,
        })
    }

    /// Get an immutable ref to the Vec of participants
    pub fn participants(&'a self) -> &'a [Person<'a, C>] {
        &self.participants
    }

    /// Use the provided Algorithm to generate the gift pairings
    pub fn generate_pairings(&'a mut self) -> Vec<Pairing<'a, C>> {
        let pairings = self.algorithm.generate_pairings(&self.participants);

        for pairing in &pairings {
            log::info!("{:?}", pairing);
        }

        pairings
    }

    /// Alt
    /*pub fn execute(participants: Vec<Person<'a, C>>, mut algorithm: A) -> Vec<GiftPairing<'a, C>> {
        let pairings = algorithm.generate_pairings(&participants);

        for pairing in &pairings {
            log::info!("{:?}", pairing);
        }

        pairings
    }*/

    /// Generate our pairings
    pub fn inform_participants(&self) {
        for person in &self.participants {
            person
                .contact()
                .send_message(&format!("Hello {}! This is a test!", person.name()));
        }
    }
}

/// Implement a consuming IntoIterator for Santa (so we can do: for p in santa {})
impl<'a, C, A> IntoIterator for Santa<'a, C, A>
where
    C: ContactMethod,
    A: Algorithm<'a, C>,
{
    type Item = Person<'a, C>;

    type IntoIter = std::vec::IntoIter<Person<'a, C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.participants.into_iter()
    }
}

/// Implement a non-consuming IntoIterator for Santa (so we can do: for p in &santa {})
impl<'a, C, A> IntoIterator for &'a Santa<'a, C, A>
where
    C: ContactMethod,
    A: Algorithm<'a, C>,
{
    type Item = <std::slice::Iter<'a, Person<'a, C>> as Iterator>::Item;

    type IntoIter = std::slice::Iter<'a, Person<'a, C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.participants.as_slice().iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorithm,
        contact::{email::Email, phone::PhoneNumber},
    };

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn new_phone_santa() {
        // First, create some participants with phone numbers
        let participants = vec![
            Person::new("Alice", PhoneNumber::new("441122334455").unwrap()),
            Person::new("Bob", PhoneNumber::new("+440987654321").unwrap()),
            Person::new("Charlie", PhoneNumber::new("441122334455").unwrap()),
        ];

        let mut prng = ChaCha8Rng::from_seed(Default::default());
        let algo = algorithm::hamiltonian::Hamiltonian::new(&mut prng);

        // Now create our Santa
        let santa = Santa::new(participants, algo).unwrap();

        assert_eq!(santa.participants().len(), 3);
    }

    #[test]
    fn new_email_santa() {
        // First, create some participants with emails
        let participants = vec![
            Person::new("Alice", Email::new("alice@email.com").unwrap()),
            Person::new("Bob", Email::new("bob@email.com").unwrap()),
            Person::new("Charlie", Email::new("charlie@email.com").unwrap()),
        ];

        let mut prng = ChaCha8Rng::from_seed(Default::default());
        let algo = algorithm::hamiltonian::Hamiltonian::new(&mut prng);

        // Now create our Santa
        let santa = Santa::new(participants, algo).unwrap();

        assert_eq!(santa.participants().len(), 3);
    }
}
