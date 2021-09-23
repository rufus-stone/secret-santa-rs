use crate::gift::Gift;
use crate::person::Person;
use crate::{algorithm::Algorithm, contact::ContactMethod};

pub type SantaResult<T> = std::result::Result<T, String>;

pub struct Santa<C, A>
where
    C: ContactMethod,
    A: Algorithm,
{
    participants: Vec<Person<C>>,
    algorithm: A,
}

impl<C, A> Santa<C, A>
where
    C: ContactMethod,
    A: Algorithm,
{
    /// Create a new Santa from the specified participants
    pub fn new(participants: Vec<Person<C>>, algorithm: A) -> SantaResult<Self> {
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
        })
    }

    /// Get an immutable ref to the Vec of participants
    pub fn participants(&self) -> &[Person<C>] {
        &self.participants
    }

    /// Use the provided Algorithm to generate the gift pairings
    pub fn generate_pairings(&self) {
        //-> Vec<Gift<C>> {
        let names: Vec<String> = self
            .participants()
            .windows(2)
            .map(|pair| pair[0].name().to_owned())
            .collect();

        println!("{:?}", names);
    }

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
impl<C, A> IntoIterator for Santa<C, A>
where
    C: ContactMethod,
    A: Algorithm,
{
    type Item = Person<C>;

    type IntoIter = std::vec::IntoIter<Person<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.participants.into_iter()
    }
}

/// Implement a non-consuming IntoIterator for Santa (so we can do: for p in &santa {})
impl<'a, C, A> IntoIterator for &'a Santa<C, A>
where
    C: ContactMethod,
    A: Algorithm,
{
    type Item = <std::slice::Iter<'a, Person<C>> as Iterator>::Item;

    type IntoIter = std::slice::Iter<'a, Person<C>>;

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
        let algo = algorithm::RandomClosedLoop::new(&mut prng);

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
        let algo = algorithm::RandomClosedLoop::new(&mut prng);

        // Now create our Santa
        let santa = Santa::new(participants, algo).unwrap();

        assert_eq!(santa.participants().len(), 3);
    }
}
