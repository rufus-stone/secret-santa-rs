use crate::{contact::ContactMethod, pairing::Pairing, person::Person};

use super::Algorithm;

#[derive(Debug, Default)]
pub struct InOrder {}

impl<'a, C: ContactMethod> Algorithm<'a, C> for InOrder {
    /// InOrder just pairs all the people in the order they appear in the participants list. The last person gets the first person
    fn generate_pairings(&mut self, participants: &'a [Person<'a, C>]) -> Vec<Pairing<'a, C>> {
        let mut pairings: Vec<Pairing<'a, C>> = participants
            .windows(2)
            .map(|pair| Pairing::new(&pair[0], &pair[1]))
            .collect();

        pairings.push(Pairing::new(
            participants.last().unwrap(),
            participants.first().unwrap(),
        ));

        pairings
    }
}

#[cfg(test)]
mod tests {
    use crate::{contact::phone::PhoneNumber, santa::Santa};

    use super::*;

    #[test]
    fn in_order_algo_phones() {
        // First, create some participants with phone numbers
        let participants = vec![
            Person::new("Alice", PhoneNumber::new("441122334455").unwrap()),
            Person::new("Bob", PhoneNumber::new("+440987654321").unwrap()),
            Person::new("Charlie", PhoneNumber::new("441122334455").unwrap()),
            Person::new("Dan", PhoneNumber::new("+449988776655").unwrap()),
            Person::new("Evelyn", PhoneNumber::new("445544668877").unwrap()),
        ];

        // Create the Santa
        let mut santa =
            Santa::new(participants.clone(), InOrder::default()).expect("Failed to create Santa!");

        // Generate the pairings
        let pairings = santa.generate_pairings();

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
    }
}
