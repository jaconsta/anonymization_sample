pub mod lib;

use crate::lib::{
    event_based::{
        sample_events::sample_cyphers::cypher_messages, search_projection::search_projection,
    },
    state_based::entity::{StateBase, Status},
};

fn main() {
    // Test de base one
    let state_based = vec![
        StateBase::new("1", "Sean", "Callahan", Status::Converted),
        StateBase::new("2", "Sarah", "Estrada", Status::Closed),
    ];
    println!("state_based={:?}", state_based);

    let events_based = search_projection::run_me_print();
    println!("events_based = {:?}", events_based);

    cypher_messages();
}
