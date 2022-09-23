#![cfg(feature = "blocking")]

use gw2lib::{model::game_mechanics::skills::Skill, Requester};

pub mod setup;

#[test]
fn single() {
    let client = setup::setup();
    let _: Skill = client.single(14375_u32).await.unwrap();
}
