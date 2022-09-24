use serde::{Deserialize, Serialize};

use crate::game_mechanics::traits::TraitId;
use crate::items::AttributeType;
pub use crate::{authenticated::characters::Profession, items};
use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type SkillId = u32;

// I would love to know if there is a better way of doing this.
// The only reason it is necessary to have two structs representing
// weapon type for the items api and the skills api is because
// shortbow is named differently in the two apis.
// items api: ShortBow
// skills api: Shortbow
////////////////////////////////////////////////
////////////////////////////////////////////////
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum SkillWeaponType {
    Axe,
    Dagger,
    Mace,
    Pistol,
    Scepter,
    Sword,
    Focus,
    Shield,
    Torch,
    Warhorn,
    Greatsword,
    Hammer,
    Longbow,
    Rifle,
    Shortbow,
    Staff,
    Harpoon,
    Spear, // I had to add spear for some skills that reference WeaponType
    Speargun,
    Trident,
    LargeBundle,
    SmallBundle,
    Toy,
    ToyTwoHanded,
    None,
}

impl From<items::WeaponType> for SkillWeaponType {
    fn from(w: items::WeaponType) -> Self {
        match w {
            items::WeaponType::Axe => SkillWeaponType::Axe,
            items::WeaponType::Dagger => SkillWeaponType::Dagger,
            items::WeaponType::Mace => SkillWeaponType::Mace,
            items::WeaponType::Pistol => SkillWeaponType::Pistol,
            items::WeaponType::Scepter => SkillWeaponType::Scepter,
            items::WeaponType::Sword => SkillWeaponType::Sword,
            items::WeaponType::Focus => SkillWeaponType::Focus,
            items::WeaponType::Shield => SkillWeaponType::Shield,
            items::WeaponType::Torch => SkillWeaponType::Torch,
            items::WeaponType::Warhorn => SkillWeaponType::Warhorn,
            items::WeaponType::Greatsword => SkillWeaponType::Greatsword,
            items::WeaponType::Hammer => SkillWeaponType::Hammer,
            items::WeaponType::LongBow => SkillWeaponType::Longbow,
            items::WeaponType::Rifle => SkillWeaponType::Rifle,
            items::WeaponType::ShortBow => SkillWeaponType::Shortbow,
            items::WeaponType::Staff => SkillWeaponType::Staff,
            items::WeaponType::Harpoon => SkillWeaponType::Harpoon,
            items::WeaponType::Spear => SkillWeaponType::Spear, // I had to add spear for some skills that reference WeaponType
            items::WeaponType::Speargun => SkillWeaponType::Speargun,
            items::WeaponType::Trident => SkillWeaponType::Trident,
            items::WeaponType::LargeBundle => SkillWeaponType::LargeBundle,
            items::WeaponType::SmallBundle => SkillWeaponType::SmallBundle,
            items::WeaponType::Toy => SkillWeaponType::Toy,
            items::WeaponType::ToyTwoHanded => SkillWeaponType::ToyTwoHanded,
            items::WeaponType::None => SkillWeaponType::None,
        }
    }
}

impl Into<items::WeaponType> for SkillWeaponType {
    fn into(self) -> items::WeaponType {
        match self {
            SkillWeaponType::Axe => items::WeaponType::Axe,
            SkillWeaponType::Dagger => items::WeaponType::Dagger,
            SkillWeaponType::Mace => items::WeaponType::Mace,
            SkillWeaponType::Pistol => items::WeaponType::Pistol,
            SkillWeaponType::Scepter => items::WeaponType::Scepter,
            SkillWeaponType::Sword => items::WeaponType::Sword,
            SkillWeaponType::Focus => items::WeaponType::Focus,
            SkillWeaponType::Shield => items::WeaponType::Shield,
            SkillWeaponType::Torch => items::WeaponType::Torch,
            SkillWeaponType::Warhorn => items::WeaponType::Warhorn,
            SkillWeaponType::Greatsword => items::WeaponType::Greatsword,
            SkillWeaponType::Hammer => items::WeaponType::Hammer,
            SkillWeaponType::Longbow => items::WeaponType::LongBow,
            SkillWeaponType::Rifle => items::WeaponType::Rifle,
            SkillWeaponType::Shortbow => items::WeaponType::ShortBow,
            SkillWeaponType::Staff => items::WeaponType::Staff,
            SkillWeaponType::Harpoon => items::WeaponType::Harpoon,
            SkillWeaponType::Spear => items::WeaponType::Spear, // I had to add spear for some skills that reference WeaponType
            SkillWeaponType::Speargun => items::WeaponType::Speargun,
            SkillWeaponType::Trident => items::WeaponType::Trident,
            SkillWeaponType::LargeBundle => items::WeaponType::LargeBundle,
            SkillWeaponType::SmallBundle => items::WeaponType::SmallBundle,
            SkillWeaponType::Toy => items::WeaponType::Toy,
            SkillWeaponType::ToyTwoHanded => items::WeaponType::ToyTwoHanded,
            SkillWeaponType::None => items::WeaponType::None,
        }
    }
}
////////////////////////////////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillType {
    Bundle,
    Elite,
    Heal,
    Profession,
    Utility,
    Weapon,
    Toolbelt,
    Monster,
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
    Heal,
    Elite,
    Toolbelt,
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
    StunBreak,
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
    Resolve,
    // Conditions
    Bleeding,
    Burning,
    Confusion,
    Poisoned,
    Torment,
    Blinded,
    Chilled,
    Crippled,
    Fear,
    Immobile,
    Slow,
    Taunt,
    Weakness,
    Vulnerability,
    // Control Effects
    Daze,
    Float,
    Knockback,
    Knockdown,
    Launch,
    Pull,
    Sink,
    Stun,
    //// Signets
    // Elementalist Signets
    #[serde(rename = "Signet of Fire")]
    SignetOfFire,
    #[serde(rename = "Signet of Water")]
    SignetOfWater,
    #[serde(rename = "Signet of Earth")]
    SignetOfEarth,
    #[serde(rename = "Signet of Air")]
    SignetOfAir,
    // Guardian Signets
    #[serde(rename = "Bane Signet")]
    BaneSignet,
    #[serde(rename = "Signet of Judgment")]
    SignetOfJudgment,
    #[serde(rename = "Signet of Wrath")]
    SignetOfWrath,
    #[serde(rename = "Signet of Mercy")]
    SignetOfMercy,
    // Mesmer-specific
    #[serde(rename = "Signet of Domination")]
    SignetOfDomination,
    #[serde(rename = "Signet of Midnight")]
    SignetOfMidnight,
    // Necromancer-specific
    #[serde(rename = "Signet of the Locust")]
    SignetOfTheLocust,
    #[serde(rename = "Signet of Spite")]
    SignetOfSpite,
    //// Profession-specific Statuses
    // Warrior-specific
    Rampage,
    // Mesmer-specific
    Mirror,
    Distortion,
    #[serde(rename = "Illusion of Life")]
    IllusionOfLife,
    #[serde(rename = "Blur")]
    Blur,
    // Guardian-specific
    #[serde(rename = "Binding Blade")]
    BindingBlade,
    #[serde(rename = "Shield of Wrath")]
    ShieldOfWrath,
    #[serde(rename = "Zealot's Flame")]
    ZealotsFlame,
    #[serde(rename = "Virtue of Resolve")]
    VirtueOfResolve,
    // Necromancer-specific
    #[serde(rename = "Grim Specter")]
    GrimSpecter,
    #[serde(rename = "Spectral Walk")]
    SpectralWalk,
    // Elementalist-specific
    #[serde(rename = "Chaos Aura")]
    ChaosAura,
    #[serde(rename = "Dark Aura")]
    DarkAura,
    #[serde(rename = "Fire Aura")]
    FireAura,
    #[serde(rename = "Frost Aura")]
    FrostAura,
    #[serde(rename = "Light Aura")]
    LightAura,
    #[serde(rename = "Magnetic Aura")]
    MagneticAura,
    #[serde(rename = "Shocking Aura")]
    ShockingAura,
    #[serde(rename = "Stone Heart")]
    StoneHeart,
    #[serde(rename = "Conjure Fire Attributes")] // Yes, these are actually different
    ConjureFireAttributes,
    #[serde(rename = "Conjure Flame Attributes")]
    ConjureFlameAttributes,
    #[serde(rename = "Conjure Earth Attributes")]
    ConjureEarthAttributes,
    #[serde(rename = "Conjure Frost Attributes")]
    ConjureFrostAttributes,
    #[serde(rename = "Conjure Lightning Attributes")]
    ConjureLightningAttributes,
    #[serde(rename = "Water Arrow")]
    WaterArrow,
    #[serde(rename = "Arcane Power")]
    ArcanePower,
    #[serde(rename = "Arcane Shield")]
    ArcaneShield,
    #[serde(rename = "Rock Barrier")]
    RockBarrier,
    #[serde(rename = "Tectonic Shift")]
    TectonicShift,
    #[serde(rename = "Renewal of Fire")]
    RenewalOfFire,
    Tornado,
    //// Race-specific
    // Norn
    Prowl,
    // Sylvari
    #[serde(rename = "Take Root")]
    TakeRoot,
    // #[serde(rename = "Conjure Magnetic Attributes")]
    // ConjureMagneticAttributes,
    // Misc
    Agony,
    Barrier,
    Invulnerability,
    Revealed,
    Stealth,
    Superspeed,
    Unblockable,
    #[serde(rename = "Fired Up!")]
    FiredUp,
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
    // These fields are not always used, ex. Crippling Shield, id: 5746
    status: Option<String>,
    description: Option<String>,
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
    // Toolbelt,
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
    GroundTargeted,
    NoUnderwater,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AttributeAdjustFact {
    text: Option<String>,
    icon: String,
    // value missing on Bloody Frenzy, id: 12424
    value: Option<u16>,
    target: AttributeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BuffFact {
    text: String,
    icon: String,
    status: Status,
    // Sometimes missing from Fact, ex. Rock Barrier, id: 5695
    description: Option<String>,
    // Sometimes duration and apply_count are not included in a Buff,
    // ex. Fiery Rush, id: 5517
    duration: Option<u8>,
    apply_count: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ComboFieldFact {
    text: String,
    icon: String,
    field_type: FieldType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ComboFinisherFact {
    text: String,
    icon: String,
    // other value of percent is f32, so this probably should be as well
    percent: f32,
    finisher_type: FinisherType,
    chance: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DamageFact {
    text: String,
    icon: String,
    hit_count: u8,
    dmg_multiplier: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DistanceFact {
    text: String,
    icon: String,
    distance: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DurationFact {
    text: String,
    icon: String,
    duration: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HealFact {
    text: String,
    icon: String,
    hit_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HealingAdjustFact {
    text: String,
    icon: String,
    hit_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct NoDataFact {
    text: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct NumberFact {
    text: String,
    icon: String,
    value: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PercentFact {
    text: String,
    icon: String,
    percent: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct RadiusFact {
    text: String,
    icon: String,
    distance: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RangeFact {
    text: String,
    icon: String,
    value: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RechargeFact {
    text: String,
    icon: String,
    value: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StunBreakFact {
    text: String,
    icon: String,
    value: bool, // this is annoying because most other value fields are numbers
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TimeFact {
    text: String,
    icon: String,
    duration: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UnblockableFact {
    text: String,
    icon: String,
    value: bool, // this is annoying because most other value fields are numbers
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
    StunBreak(StunBreakFact),
    Time(TimeFact),
    Unblockable(UnblockableFact),
}

impl From<Fact> for FactType {
    fn from(f: Fact) -> Self {
        match f {
            Fact::AttributeAdjust(_) => FactType::AttributeAdjust,
            Fact::Buff(_) => FactType::Buff,
            Fact::ComboField(_) => FactType::ComboField,
            Fact::ComboFinisher(_) => FactType::ComboFinisher,
            Fact::Damage(_) => FactType::Damage,
            Fact::Distance(_) => FactType::Distance,
            Fact::Duration(_) => FactType::Duration,
            Fact::Heal(_) => FactType::Heal,
            Fact::HealingAdjust(_) => FactType::HealingAdjust,
            Fact::NoData(_) => FactType::NoData,
            Fact::Number(_) => FactType::Number,
            Fact::Percent(_) => FactType::Percent,
            Fact::PrefixedBuff(_) => FactType::PrefixedBuff,
            Fact::Radius(_) => FactType::Radius,
            Fact::Range(_) => FactType::Range,
            Fact::Recharge(_) => FactType::Recharge,
            Fact::StunBreak(_) => FactType::StunBreak,
            Fact::Time(_) => FactType::Time,
            Fact::Unblockable(_) => FactType::Unblockable,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub chat_link: String,
    // not all skills have a 'type' field. For example; Binding Roots, id: 1279
    #[serde(rename = "type")]
    pub _type: Option<SkillType>,
    pub weapon_type: Option<SkillWeaponType>,
    // not all skills have a 'professions' field. For example; Binding Roots, id: 1279
    pub professions: Option<Vec<Profession>>,
    // not all skills have a 'slot' field. For example; Binding Roots, id: 1279
    pub slot: Option<Slot>,
    pub traited_facts: Option<Vec<TraitedFact>>,
    pub categores: Option<Category>,
    pub attunement: Option<Attunement>,
    pub cost: Option<u8>,
    pub dual_wield: Option<SkillWeaponType>,
    pub flip_skill: Option<SkillId>,
    pub initiative: Option<u8>,
    pub next_chain: Option<SkillId>,
    pub prev_chain: Option<SkillId>,
    pub transform_skills: Option<Vec<SkillId>>,
    pub bundle_skills: Option<Vec<SkillId>>,
    pub toolbelt_skill: Option<SkillId>,
    pub flags: Option<Vec<SkillFlag>>,
    // not al skills have a 'facts' field, for example Fire Attunement, id: 5492
    pub facts: Option<Vec<Fact>>,
}

/// This does not work,
/// returns Error("missing field `text`"...
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
// pub struct TraitedFact {
//     pub text: String,
//     #[serde(rename = "type", flatten)]
//     pub _type: Fact,
//     pub requires_trait: TraitId,
//     /// array index of Fact
//     pub overrides: Option<usize>,
// }

/// This does not work either,
/// returns Error("invalid type: string \"Damage\", expected internally tagged
///     enum Fact"
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
// pub struct TraitedFact {
//     pub text: String,
//     #[serde(rename = "type")]
//     pub _type: Fact,
//     pub requires_trait: TraitId,
//     /// array index of Fact
//     pub overrides: Option<usize>,
// }

/// This also does not work.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
// pub struct TraitedFact {
//     #[serde(rename = "type", flatten)]
//     pub _type: Fact,
//     pub requires_trait: TraitId,
//     /// array index of Fact
//     pub overrides: Option<usize>,
// }

/// This works, but is there a better way?
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraitedFact {
    pub text: String,
    #[serde(rename = "type")]
    pub _type: FactType,
    pub requires_trait: TraitId,
    /// array index of Fact
    pub overrides: Option<usize>,

    /// these are all the fields Facts can have
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<FieldType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finisher_type: Option<FinisherType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmg_multiplier: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharge: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<Prefix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<AttributeType>,
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
