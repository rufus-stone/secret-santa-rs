use santa_core as snt;
use simple_logger::SimpleLogger;
use snt::algorithm::{hamiltonian::*, inorder::*};
use snt::contact::phone::*;
use snt::person::*;
use snt::santa::Santa;

fn main() {
    // Start logger
    SimpleLogger::new().init().unwrap();

    // First, create some participants with phone numbers
    let participants = vec![
        Person::new("Alice", PhoneNumber::new("441122334455").unwrap()),
        Person::new("Bob", PhoneNumber::new("+440987654321").unwrap()),
        Person::new("Charlie", PhoneNumber::new("+441122334455").unwrap()),
    ];

    // Now create our Santa
    //let santa = Santa::new(participants, RandomClosedLoop::default()).expect("Failed to create Santa!");
    let mut santa = Santa::new(participants, InOrder::default()).expect("Failed to create Santa!");
    log::info!("{:?}", &santa);

    let pairings = santa.generate_pairings();

    //santa.inform_participants();
}
