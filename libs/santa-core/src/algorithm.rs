use rand::RngCore;

use crate::{contact::ContactMethod, gift::Gift};

pub trait Algorithm {
    fn generate_pairings();
}

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

impl Algorithm for RandomClosedLoop<'_> {
    fn generate_pairings() {
        todo!()
    }
}
