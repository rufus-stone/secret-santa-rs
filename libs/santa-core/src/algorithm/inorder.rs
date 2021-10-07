use crate::{contact::ContactMethod, pairing::Pairing, person::Person};

use super::Algorithm;

#[derive(Debug, Default)]
pub struct InOrder {}

impl<'a, C: ContactMethod> Algorithm<'a, C> for InOrder {
    /// InOrder just pairs all the people in the order they appear in the participants list. The last person gets the first person
    fn generate_pairings(&self, participants: &'a [Person<'a, C>]) -> Vec<Pairing<'a, C>> {
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
