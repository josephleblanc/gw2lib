#![cfg(feature = "blocking")]

use gw2lib::{
    model::items::itemstats::{ItemStats, StatsId},
    Requester,
};

pub mod setup;

#[test]
fn ritualist() {
    let client = setup::setup();
    let _: ItemStats = client.single(1717_u32).unwrap();
}

#[test]
fn all_itemstats() {
    let client = setup::setup();
    let _: Vec<ItemStats> = client.all().unwrap();
}
