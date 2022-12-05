use crate::{
    // authenticated::characters::Profession,
    game_mechanics::{
        skills::{Attunement, SkillId, SkillType, Slot},
        specializations::{Specialization, SpecializationId},
        traits::TraitId,
    },
    items::WeaponType,
    BulkEndpoint,
    Endpoint,
    EndpointWithId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Note to self: Try to make the ProfessionId into an enum that will still work
// with the client function. Perhaps try using From<T> on String.
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub enum ProfessionId {
//     Guardian,
//     Warrior,
//     Engineer,
//     Ranger,
//     Thief,
//     Elementalist,
//     Mesmer,
//     Necromancer,
//     Revenant,
// }
pub type ProfessionId = String;
pub type TrainingId = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrainingCategory {
    Skills,
    Specializations,
    EliteSpecializations,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrackType {
    Trait,
    Skill,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfessionFlag {
    NoRacialSkills,
    NoWeaponSwap,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfessionWeaponFlags {
    Mainhand,
    Offhand,
    TwoHand,
    Aquatic,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Track {
    cost: u16,
    #[serde(rename = "type")]
    _type: TrackType,
    skill_id: Option<SkillId>,
    trait_id: Option<TraitId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Training {
    id: TrainingId,
    category: TrainingCategory,
    name: String,
    track: Vec<Track>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfessionSkill {
    id: SkillId,
    slot: Slot,
    offhand: Option<WeaponType>,
    attunement: Option<Attunement>,
    source: Option<ProfessionId>,
    #[serde(rename = "type")]
    _type: Option<SkillType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfessionWeaponDetails {
    specialization: Option<SpecializationId>,
    flags: Vec<ProfessionWeaponFlags>,
    skills: Vec<ProfessionSkill>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Profession {
    id: ProfessionId,
    name: String,
    code: Option<u32>, // format this into a build link later
    icon: String,
    icon_big: String,
    specializations: Vec<SpecializationId>,
    training: Vec<Training>,
    weapons: HashMap<WeaponType, ProfessionWeaponDetails>,
    flags: Vec<ProfessionFlag>,
    skills_by_palette: Option<Vec<(u32, SkillId)>>,
    skills: Vec<ProfessionSkill>,
}

impl Endpoint for Profession {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/professions";
    const VERSION: &'static str = "2022-07-22T00:00:00.000Z";
}

impl EndpointWithId for Profession {
    type IdType = ProfessionId;
}

impl BulkEndpoint for Profession {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
