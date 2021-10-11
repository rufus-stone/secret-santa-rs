use crate::{contact::ContactMethod, pairing::Pairing, person::Person};

pub trait Algorithm<'a, C: ContactMethod> {
    fn generate_pairings(&mut self, participants: &'a [Person<'a, C>]) -> Vec<Pairing<'a, C>>;
}

pub mod hamiltonian;
pub mod inorder;
