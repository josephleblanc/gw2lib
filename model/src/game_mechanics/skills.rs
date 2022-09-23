use serde::{Deserialize, Serialize};

use crate::game_mechanics::traits::TraitId;
use crate::items::AttributeType;
pub use crate::{authenticated::characters::Profession, items::WeaponType};
use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type SkillId = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillType {
    Bundle,
    Elite,
    Heal,
    Profession,
    Utility,
    Weapon,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Slot {
    Downed_1,
    Downed_2,
    Downed_3,
    Downed_4,
    Pet,
    Profession_1,
    Profession_2,
    Profession_3,
    Profession_4,
    Profession_5,
    Utility,
    Weapon_1,
    Weapon_2,
    Weapon_3,
    Weapon_4,
    Weapon_5,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FactType {
    AttributeAdjust,
    Buff,
    ComboField,
    ComboFinisher,
    Damage,
    Distance,
    Duration,
    Heal,
    HealingAdjust,
    NoData,
    Number,
    Percent,
    PrefixedBuff,
    Radius,
    Range,
    Recharge,
    Time,
    Unblockable,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    // Boons
    Aegis,
    Alacrity,
    Fury,
    Might,
    Protection,
    Quickness,
    Regeneration,
    Resistance,
    Resolution,
    Stability,
    Swiftness,
    Vigor,
    // Conditions
    Bleed,
    Burning,
    Confusion,
    Poisoned,
    Torment,
    Blinded,
    Chilled,
    Crippled,
    Fear,
    Immobilized,
    Slow,
    Taunt,
    Weakness,
    Vulnerability,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldType {
    Dark,
    Ethereal,
    Fire,
    Ice,
    Light,
    Lightning,
    Poison,
    Smoke,
    Water,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FinisherType {
    Blast,
    Leap,
    Projectile,
    Whirl,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prefix {
    text: String,
    icon: String,
    status: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Category {
    Cantrip,
    Deception,
    Elixir,
    Glyph,
    Mantra,
    Meditation,
    Physical,
    Shout,
    Signet,
    Stance,
    Trap,
    Well,
    Consecration,
    SpiritWeapon,
    Symbol,
    Tome,
    Virtue,
    Ward,
    CitadelOrder,
    Consume,
    Facet,
    Legend,
    LegendaryAlliance,
    LegendaryAssassin,
    LegendaryCentaur,
    LegendaryDemon,
    LegendaryDragon,
    LegendaryDwarf,
    LegendaryRenegade,
    Armament,
    Banner,
    Burst,
    PrimalBurst,
    Rage,
    EngineeringKit,
    Exceed,
    Gadget,
    PhotonForge,
    Toolbelt,
    Turret,
    Beast,
    CelestialAvatar,
    Pet,
    Command,
    Spirit,
    Survival,
    UnleashedAmbush,
    DualWield,
    Kneel,
    Preparation,
    StealthAttack,
    Stolenskill,
    Trick,
    Venom,
    Arcane,
    Attunement,
    Augment,
    Conjure,
    DualAttack,
    Overload,
    Ambush,
    Bladesong,
    #[serde(rename = "Clone")]
    _Clone, // Did this need to be changed?
    Glamour,
    Manipulation,
    Phantasm,
    Psionic,
    Shatter,
    Corruption,
    Mark,
    Minion,
    Punishment,
    Shade,
    Spectral,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Attunement {
    Fire,
    Water,
    Air,
    Earth,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SkillFlag {
    GroundedTarget,
    NoUnderwater,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeAdjustFact {
    text: String,
    icon: String,
    value: u16,
    target: AttributeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuffFact {
    text: String,
    icon: String,
    duration: u8,
    status: Status,
    description: String,
    apply_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComboFieldFact {
    text: String,
    field_type: FieldType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComboFinisherFact {
    text: String,
    percent: u8,
    finisher_type: FinisherType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DamageFact {
    text: String,
    icon: String,
    hit_count: u8,
    dmg_multiplier: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DistanceFact {
    text: String,
    icon: String,
    distance: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DurationFact {
    text: String,
    icon: String,
    duration: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealFact {
    text: String,
    icont: String,
    hit_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealingAdjustFact {
    text: String,
    icon: String,
    hit_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoDataFact {
    text: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NumberFact {
    text: String,
    icon: String,
    value: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PercentFact {
    text: String,
    icon: String,
    value: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefixedBuffFact {
    text: String,
    icon: String,
    duration: u8,
    status: Status,
    description: String,
    apply_count: u8,
    prefix: Prefix,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiusFact {
    text: String,
    icon: String,
    distance: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RangeFact {
    text: String,
    icon: String,
    value: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RechargeFact {
    text: String,
    recharge: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StunBreakFact {
    text: String,
    icon: String,
    value: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeFact {
    text: String,
    icon: String,
    duration: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnblockableFact {
    text: String,
    value: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Fact {
    AttributeAdjust(AttributeAdjustFact),
    Buff(BuffFact),
    ComboField(ComboFieldFact),
    ComboFinisher(ComboFinisherFact),
    Damage(DamageFact),
    Distance(DistanceFact),
    Duration(DurationFact),
    Heal(HealFact),
    HealingAdjust(HealingAdjustFact),
    NoData(NoDataFact),
    Number(NumberFact),
    Percent(PercentFact),
    PrefixedBuff(PrefixedBuffFact),
    Radius(RadiusFact),
    Range(RangeFact),
    Recharge(RechargeFact),
    Time(TimeFact),
    Unblockable(UnblockableFact),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub text: String,
    pub description: String,
    pub icon: Option<String>,
    pub chat_link: String,
    #[serde(rename = "type")]
    pub _type: SkillType,
    pub weapon_type: Option<WeaponType>,
    pub professions: Vec<Profession>,
    pub slot: Slot,
    pub facts: Vec<Fact>,
    pub traited_facts: Option<TraitedFact>,
    pub categores: Category,
    pub attunement: Option<Attunement>,
    pub cost: Option<u8>,
    pub dual_weild: Option<WeaponType>,
    pub flip_skill: Option<SkillId>,
    pub initiative: Option<u8>,
    pub next_chain: Option<SkillId>,
    pub prev_chain: Option<SkillId>,
    pub transform_skills: Option<Vec<SkillId>>,
    pub bundle_skills: Option<Vec<SkillId>>,
    pub toolbelt_skill: Option<SkillId>,
    pub flags: Option<Vec<SkillFlag>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraitedFact {
    #[serde(flatten)]
    pub fact: Fact,
    pub requires_trait: TraitId,
    /// array index of Fact
    pub overrides: Option<u8>,
}

impl Endpoint for Skill {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/skills";
    const VERSION: &'static str = "2022-07-22T00:00:00.000Z";
}

impl EndpointWithId for Skill {
    type IdType = SkillId;
}

impl BulkEndpoint for Skill {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
