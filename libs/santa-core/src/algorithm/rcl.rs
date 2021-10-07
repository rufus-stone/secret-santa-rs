use rand::RngCore;

use crate::{contact::ContactMethod, pairing::Pairing, person::Person};

use super::Algorithm;

pub struct RandomClosedLoop<'a> {
    prng: Box<dyn RngCore + 'a>,
}

impl<'a> RandomClosedLoop<'a> {
    /// Create a new RandomClosedLoop with the specified prng
    pub fn new(prng: &'a mut dyn RngCore) -> Self {
        Self {
            prng: Box::new(prng),
        }
    }
}

impl Default for RandomClosedLoop<'_> {
    fn default() -> Self {
        Self {
            prng: Box::new(rand::thread_rng()),
        }
    }
}

impl<'a, C: ContactMethod> Algorithm<'a, C> for RandomClosedLoop<'a> {
    fn generate_pairings(&self, participants: &'a [Person<'a, C>]) -> Vec<Pairing<'a, C>> {
        todo!()
    }
}
