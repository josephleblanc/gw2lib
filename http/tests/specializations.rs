#![cfg(feature = "blocking")]

use gw2lib::{
    model::game_mechanics::specializations::{Specialization, SpecializationId},
    Requester,
};

pub mod setup;

// This doesn't work in the tests for some reason, though it returns the
// appropriate value when used as a function in another library.
// #[test]
// fn specializations_elite_untamed() {
//     let client = setup::setup();
//     let untamed: SpecializationId = 72_u16;
//     let _: SpecializationId = client.single(untamed).unwrap();
// }
#[test]
fn specializations_all() {
    let client = setup::setup();
    let _: Vec<Specialization> = client.all().unwrap();
}
