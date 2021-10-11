use rand::seq::SliceRandom;
use rand::RngCore;

use crate::{contact::ContactMethod, pairing::Pairing, person::Person};

use super::Algorithm;

pub struct Hamiltonian<'a> {
    prng: Box<dyn RngCore + 'a>,
}

impl<'a> Hamiltonian<'a> {
    /// Create a new Hamiltonian with the specified prng
    pub fn new(prng: &'a mut dyn RngCore) -> Self {
        Self {
            prng: Box::new(prng),
        }
    }
}

impl Default for Hamiltonian<'_> {
    fn default() -> Self {
        Self {
            prng: Box::new(rand::thread_rng()),
        }
    }
}

impl<'a, C: ContactMethod> Algorithm<'a, C> for Hamiltonian<'a> {
    fn generate_pairings(&mut self, participants: &'a [Person<'a, C>]) -> Vec<Pairing<'a, C>> {
        // First, use the prng to shuffle the participants
        let mut order: Vec<usize> = (0..participants.len() - 1).collect();
        order.shuffle(&mut self.prng);

        // Now generate pairings based on the shuffled order
        let mut pairings: Vec<Pairing<'a, C>> = order
            .windows(2)
            .map(|index| Pairing::new(&participants[index[0]], &participants[index[1]]))
            .collect();

        pairings.push(Pairing::new(
            &participants[*order.last().unwrap()],
            &participants[*order.first().unwrap()],
        ));

        pairings
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{contact::phone::PhoneNumber, santa::Santa};

    use super::*;

    #[test]
    fn hamiltonian_algo_phones() {
        // First, create some participants with phone numbers
        let participants = vec![
            Person::new("Alice", PhoneNumber::new("441122334455").unwrap()),
            Person::new("Bob", PhoneNumber::new("+440987654321").unwrap()),
            Person::new("Charlie", PhoneNumber::new("441122334455").unwrap()),
            Person::new("Dan", PhoneNumber::new("+449988776655").unwrap()),
            Person::new("Evelyn", PhoneNumber::new("445544668877").unwrap()),
        ];

        // Create a prng
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Create the Santa
        let mut santa = Santa::new(participants.clone(), Hamiltonian::new(&mut prng))
            .expect("Failed to create Santa!");

        // Generate the pairings
        /*let pairings = santa.generate_pairings();

        // Check that the number of pairings is correct
        assert_eq!(pairings.len(), 5);

        // Check that the pairings are all in order
        assert_eq!(pairings[0].giver(), &participants[0]);
        assert_eq!(pairings[0].recipient(), &participants[1]);
        assert_eq!(pairings[1].giver(), &participants[1]);
        assert_eq!(pairings[1].recipient(), &participants[2]);
        assert_eq!(pairings[2].giver(), &participants[2]);
        assert_eq!(pairings[2].recipient(), &participants[3]);
        assert_eq!(pairings[3].giver(), &participants[3]);
        assert_eq!(pairings[3].recipient(), &participants[4]);
        assert_eq!(pairings[4].giver(), &participants[4]);
        assert_eq!(pairings[4].recipient(), &participants[0]);
        */
    }
}
