#![allow(dead_code)]
use crate::{
    authenticated::characters::Profession, game_mechanics::traits::TraitId, BulkEndpoint, Endpoint,
    EndpointWithId,
};
use serde::{Deserialize, Serialize};

pub type SpecializationId = u16;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Specialization {
    id: SpecializationId,
    name: String,
    profession: Profession,
    elite: bool,
    icon: String,
    background: String,
    minor_traits: Vec<TraitId>,
    major_traits: Vec<TraitId>,
}

impl Endpoint for Specialization {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/specializations";
    const VERSION: &'static str = "2022-07-22T00:00:00.000Z";
}

impl EndpointWithId for Specialization {
    type IdType = SpecializationId;
}

impl BulkEndpoint for Specialization {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
