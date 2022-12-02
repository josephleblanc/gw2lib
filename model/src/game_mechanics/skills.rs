use serde::{Deserialize, Deserializer, Serialize};

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
    Toolbelt,
    Monster,
    Pet,
    // Someone said there is a Transform type as well.
    // Wait until I can verify to include it.
    // Transform,
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
    // Stunbreak is not listed on the wiki as one of the possible fields, but
    // you can see an example of it being used in Glyph of Elemental Power,
    // id: 5506
    StunBreak,
    Time,
    Unblockable,
    UnTyped,
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
    // Ranger-specific
    #[serde(rename = "Signet of the Wild")]
    SignetOfTheWild,
    #[serde(rename = "Signet of Stone")]
    SignetOfStone,
    #[serde(rename = "Signet of Renewal")]
    SignetOfRenewal,
    #[serde(rename = "Signet of the Hunt")]
    SignetOfTheHunt,
    // Thief-specific
    #[serde(rename = "Assassin's Signet")]
    AssassinsSignet,
    #[serde(rename = "Signet of Agility")]
    SignetOfAgility,
    // Warrior-Specific
    #[serde(rename = "Signet of Rage")]
    SignetOfRage,
    #[serde(rename = "Healing Signet")]
    HealingSignet,
    #[serde(rename = "Signet of Might")]
    SignetOfMight,
    #[serde(rename = "Signet of Fury")]
    SignetOfFury,
    #[serde(rename = "Dolyak Signet")]
    DolyakSignet,
    #[serde(rename = "Signet of Stamina")]
    SignetOfStamina,
    //// Profession-specific Statuses
    // Warrior-specific
    Rampage,
    #[serde(rename = "Balanced Stance")]
    BalancedStance,
    #[serde(rename = "Enduring Pain")]
    EnduringPain,
    #[serde(rename = "Positive Flow")]
    PositiveFlow,
    #[serde(rename = "Berserker's Stance")]
    BerserkersStance,
    #[serde(rename = "Defiant Stance")]
    DefiantStance,
    // Mesmer-specific
    Mirror,
    Distortion,
    #[serde(rename = "Illusion of Life")]
    IllusionOfLife,
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
    #[serde(rename = "Litany of Wrath")]
    LitanyOfWrath,
    // Necromancer-specific
    #[serde(rename = "Grim Specter")]
    GrimSpecter,
    #[serde(rename = "Spectral Walk")]
    SpectralWalk,
    #[serde(rename = "Vampiric Mark")]
    VampiricMark,
    // Ranger-specific
    #[serde(rename = "Strength of the Pack")]
    StrengthOfThePack,
    #[serde(rename = "Counter Ready")]
    CounterReady,
    #[serde(rename = "Attack of Opportunity")]
    AttackOfOpportunity,
    #[serde(rename = "Sharpening Stone")]
    SharpeningStone,
    #[serde(rename = r#""Sic 'Em!""#)]
    SicEm,
    #[serde(rename = "Feeding Frenzy")]
    FeedingFrenzy,
    #[serde(rename = "Serpent's Preparation")]
    SerpentsPreparation, // granted by Monarch's Leap, id: 12622
    // Thief-specific
    Repeater,
    #[serde(rename = "Hooked Spear")]
    HookedSpear,
    #[serde(rename = "Spider Venom")]
    SpiderVenom,
    #[serde(rename = "Skale Venom")]
    SkaleVenom,
    #[serde(rename = "Basilisk Venom")]
    BasiliskVenom,
    #[serde(rename = "Devourer Venom")]
    DevourerVenom,
    #[serde(rename = "Ice Drake Venom")]
    IceDrakeVenom,
    #[serde(rename = "Skelk Venom")]
    SkelkVenom,
    // Engineer-specific
    Plague,
    #[serde(rename = "A.E.D.")]
    Aed,

    // Elementalist-specific
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
    Whirlpool,
    //// Race-specific
    // Norn
    Prowl,
    // Sylvari
    #[serde(rename = "Take Root")]
    TakeRoot,
    //// Aura
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
    //// ComboField
    // These are used in some skills to specify the type of field created,
    // e.g. Rift of Pain, id: 50390
    #[serde(rename = "Combo Field: Dark")]
    ComboFieldDark,
    #[serde(rename = "Combo Field: Lightning")]
    ComboFieldLightning,
    #[serde(rename = "Combo Field: Light")]
    ComboFieldLight,
    #[serde(rename = "Combo Field: Fire")]
    ComboFieldFire,
    #[serde(rename = "Combo Field: Water")]
    ComboFieldWater,

    //// Misc
    Agony,
    Barrier,
    Invulnerability,
    Revealed,
    Stealth,
    Superspeed,
    Unblockable,
    #[serde(rename = "Fired Up!")]
    FiredUp,

    // Found using a program, go through them later to sort them into their
    // professions as above, where applicable
    Afterburner,
    #[serde(rename = "Aquatic Stance")]
    AquaticStance,
    #[serde(rename = "Barrier Signet")]
    BarrierSignet,
    #[serde(rename = "Bear Stance")]
    BearStance,
    #[serde(rename = "Berserker's Power")]
    BerserkersPower,
    Blight,
    #[serde(rename = "Breakrazor's Bastion")]
    BreakrazorsBastion,
    #[serde(rename = "Burst of Strength")]
    BurstOfStrength,
    #[serde(rename = "Chaos Corrosion")]
    ChaosCorrosion,
    #[serde(rename = "Cooling Vapor")]
    CoolingVapor,
    #[serde(rename = "Conjured Barrier")]
    ConjuredBarrier,
    #[serde(rename = "Crashing Courage")]
    CrashingCourage,
    #[serde(rename = "Crescent Wind")]
    CrescentWind,
    #[serde(rename = "Death's Advance")]
    DeathsAdvance,
    #[serde(rename = "Defy Pain")]
    DefyPain,
    Disenchantment,
    #[serde(rename = "Dolyak Stance")]
    DolyakStance,
    Echo,
    #[serde(rename = "Enfeebled Force")]
    EnfeebledForce,
    #[serde(rename = "Facet of Chaos")]
    FacetOfChaos,
    #[serde(rename = "Facet of Darkness")]
    FacetOfDarkness,
    #[serde(rename = "Facet of Elements")]
    FacetOfElements,
    #[serde(rename = "Facet of Light")]
    FacetOfLight,
    #[serde(rename = "Facet of Nature")]
    FacetOfNature,
    #[allow(non_camel_case_types)]
    #[serde(rename = "Facet of Nature—Assassin")]
    FacetOfNature_Assassin,
    #[allow(non_camel_case_types)]
    #[serde(rename = "Facet of Nature—Centaur")]
    FacetOfNature_Centaur,
    #[allow(non_camel_case_types)]
    #[serde(rename = "Facet of Nature—Dwarf")]
    FacetOfNature_Dwarf,
    #[allow(non_camel_case_types)]
    #[serde(rename = "Facet of Nature—Dragon")]
    FacetOfNature_Dragon,
    #[allow(non_camel_case_types)]
    #[serde(rename = "Facet of Nature—Demon")]
    FacetOfNature_Demon,
    #[serde(rename = "Facet of Strength")]
    FacetOfStrength,
    #[serde(rename = "Flame Wheel")]
    FlameWheel,
    #[serde(rename = "Flowing Resolve")]
    FlowingResolve,
    #[serde(rename = "Force Signet")]
    ForceSignet,
    #[serde(rename = "Forced Engagement")]
    ForcedEngagement,
    #[serde(rename = "Forerunner of Death")]
    ForerunnerOfDeath,
    #[serde(rename = "Forest's Fortification")]
    ForestsFortification,
    Fractured,
    #[serde(rename = "Griffon Stance")]
    GriffonStance,
    #[serde(rename = "Grinding Stones")]
    GrindingStones,
    #[serde(rename = "Icy Coil")]
    IcyCoil,
    #[serde(rename = "Igniting Brand")]
    IgnitingBrand,
    #[serde(rename = "Imperial Guard")]
    ImperialGuard,
    #[serde(rename = "Impossible Odds")]
    ImpossibleOdds,
    #[serde(rename = "Improved Kalla's Fervor")]
    ImprovedKallasFervor,
    #[serde(rename = "Incoming conditions are ignored.")]
    IncomingConditionsAreIgnored,
    #[serde(rename = "Infuse Light")]
    InfuseLight,
    Justice,
    #[serde(rename = "Kalla's Fervor")]
    KallasFervor,
    #[serde(rename = "Kinetic Abundance")]
    KineticAbundance,
    #[serde(rename = "Mirage Cloak")]
    MirageCloak,
    #[serde(rename = "Moa Stance")]
    MoaStance,
    #[serde(rename = "Molten Armor")]
    MoltenArmor,
    Morphed,
    #[serde(rename = "One Wolf Pack")]
    OneWolfPack,
    #[serde(rename = "Overcharged Cartridges")]
    OverchargedCartridges,
    #[serde(rename = "Palm Strike")]
    PalmStrike,
    #[serde(rename = "Perfect Weave")]
    PerfectWeave,
    #[serde(rename = "Perilous Gift")]
    PerilousGift,
    #[serde(rename = "Photon Forge")]
    PhotonForge,
    #[serde(rename = "Primordial Stance")]
    PrimordialStance,
    #[serde(rename = "Pulmonary Impact")]
    PulmonaryImpact,
    #[serde(rename = "Radiant Blindness")]
    RadiantBlindness,
    #[serde(rename = "Razorclaw's Rage")]
    RazorclawsRage,
    Rebound,
    Repose,
    #[serde(rename = "Reversal of Fortune")]
    ReversalOfFortune,
    #[serde(rename = "Rite of the Great Dwarf")]
    RiteOfTheGreatDwarf,
    #[serde(rename = "Rock Guard")]
    RockGuard,
    #[serde(rename = "Rocky Loop")]
    RockyLoop,
    #[serde(rename = "Saint of zu Heltzer")]
    SaintOfZuHeltzer,
    #[serde(rename = "Sharpen Spines")]
    SharpenSpines,
    #[serde(rename = "Sight beyond Sight")]
    SightBeyondSight,
    #[serde(rename = "Soulcleave's Summit")]
    SoulcleavesSummit,
    #[serde(rename = "Spectrum Shield")]
    SpectrumShield,
    #[serde(rename = "Static Charge")]
    StaticCharge,
    #[serde(rename = "Stim State")]
    StimState,
    #[serde(rename = "Stone Resonance")]
    StoneResonance,
    #[serde(rename = "Superconducting Signet")]
    SuperconductingSignet,
    #[serde(rename = "Tactical Reload")]
    TacticalReload,
    #[serde(rename = "Time Echo")]
    TimeEcho,
    #[serde(rename = "Tome of Resolve")]
    TomeOfResolve,
    Tranquil,
    #[serde(rename = "True Nature")]
    TrueNature,
    #[serde(rename = "Unbridled Chaos")]
    UnbridledChaos,
    #[serde(rename = "Unbridled Fear")]
    UnbridledFear,
    #[serde(rename = "Unflinching Fortitude")]
    UnflinchingFortitude,
    Unravel,
    #[serde(rename = "Urn of Saint Viktor")]
    UrnOfSaintViktor,
    #[serde(rename = "Vampiric Infection")]
    VampiricInfection,
    #[serde(rename = "Vengeful Hammers")]
    VengefulHammers,
    #[serde(rename = "Violent Currents")]
    ViolentCurrents,
    #[serde(rename = "Virtue of Courage")]
    VirtueOfCourage,
    #[serde(rename = "Vulture Stance")]
    VultureStance,
    #[serde(rename = "Watchful Eye")]
    WatchfulEye,
    Waterlogged,
    #[serde(rename = "Weave Self")]
    WeaveSelf,
    #[serde(rename = "Woven Earth")]
    WovenEarth,
    #[serde(rename = "Woven Air")]
    WovenAir,
    #[serde(rename = "Woven Fire")]
    WovenFire,
    #[serde(rename = "Woven Water")]
    WovenWater,
    #[serde(rename = "You do not lose health from being downed.")]
    YoDoNotLoseHealthFromBeingDowned,
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
#[serde(tag = "type")]
pub struct AttributeAdjustFact {
    text: Option<String>,
    icon: String,
    // value missing on Bloody Frenzy, id: 12424
    value: Option<u16>,
    target: AttributeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
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
#[serde(tag = "type")]
pub struct ComboFieldFact {
    text: String,
    icon: String,
    field_type: FieldType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct ComboFinisherFact {
    text: String,
    icon: String,
    // other value of percent (in PercentFact struct) needed to be f32,
    // so this probably should as well.
    // Test this later.
    percent: f32,
    finisher_type: FinisherType,
    chance: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct DamageFact {
    text: String,
    icon: String,
    hit_count: u8,
    dmg_multiplier: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct DistanceFact {
    text: String,
    icon: String,
    distance: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct DurationFact {
    text: String,
    icon: String,
    duration: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct HealFact {
    text: String,
    icon: String,
    hit_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct HealingAdjustFact {
    text: String,
    icon: String,
    hit_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct NoDataFact {
    text: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct NumberFact {
    // text field not always present, e.g. Tides of Time, id: 30643
    text: Option<String>,
    icon: String,
    value: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct PercentFact {
    text: String,
    icon: String,
    // Some Percent facts have 'value' fields instead of the more common
    // 'percent' field.
    // e.g. Engage Photon Forge
    #[serde(alias = "value")]
    percent: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct PrefixedBuffFact {
    text: String,
    icon: String,
    duration: u8,
    status: Status,
    // description field not always present, e.g. True Nature, id: 29393
    description: Option<String>,
    apply_count: u8,
    prefix: Prefix,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct RadiusFact {
    text: String,
    icon: String,
    distance: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename = "Range")]
pub struct RangeFact {
    text: String,
    icon: String,
    value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct RechargeFact {
    text: String,
    icon: String,
    value: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct StunBreakFact {
    text: String,
    icon: String,
    value: bool, // this is annoying because most other value fields are numbers
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct TimeFact {
    text: String,
    icon: String,
    duration: u16, // duration needing over u8 surprised me.
                   // Some skills have a duration over 255.
                   // Example: Prepare Thousand Needles, id: 13026
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct UnblockableFact {
    text: String,
    icon: String,
    value: bool, // this is annoying because most other value fields are numbers
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
// this is not marked with serde(tag = "type"), which is how it is chosen from
// the untagged Fact enum when there is no "type" key provided by the api
pub struct UnTypedFact {
    text: String,
    icon: String,
    // other value of percent (in PercentFact struct) needed to be f32,
    // so this probably should as well.
    // Test this later.
    percent: u16,
}

/// Trying to get a custom deserialize to work with Fact struct to handle
/// skill facts with no 'type' entry.
use serde::de;
impl<'de> Deserialize<'de> for Fact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_json::Value;
        #[derive(Deserialize, Debug)]
        struct FactHelper {
            #[serde(rename = "type")]
            fact_type: Option<FactType>,
        }

        let v = Value::deserialize(deserializer)?;
        let helper = FactHelper::deserialize(&v).map_err(de::Error::custom)?;
        // println!("{:?}", helper.fact_type);
        match helper.fact_type {
            Some(fact_type) => {
                if fact_type == FactType::AttributeAdjust {
                    let fact = AttributeAdjustFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::AttributeAdjust(fact))
                } else if fact_type == FactType::Buff {
                    let fact = BuffFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Buff(fact))
                } else if fact_type == FactType::ComboField {
                    let fact = ComboFieldFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::ComboField(fact))
                } else if fact_type == FactType::ComboFinisher {
                    let fact = ComboFinisherFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::ComboFinisher(fact))
                } else if fact_type == FactType::Damage {
                    let damage_fact = DamageFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Damage(damage_fact))
                } else if fact_type == FactType::Distance {
                    let fact = DistanceFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Distance(fact))
                } else if fact_type == FactType::Duration {
                    let fact = DurationFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Duration(fact))
                } else if fact_type == FactType::Heal {
                    let fact = HealFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Heal(fact))
                } else if fact_type == FactType::HealingAdjust {
                    let fact = HealingAdjustFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::HealingAdjust(fact))
                } else if fact_type == FactType::NoData {
                    let fact = NoDataFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::NoData(fact))
                } else if fact_type == FactType::Number {
                    let fact = NumberFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Number(fact))
                } else if fact_type == FactType::Percent {
                    let fact = PercentFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Percent(fact))
                } else if fact_type == FactType::PrefixedBuff {
                    let fact = PrefixedBuffFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::PrefixedBuff(fact))
                } else if fact_type == FactType::Radius {
                    let fact = RadiusFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Radius(fact))
                } else if fact_type == FactType::Range {
                    let fact = RangeFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Range(fact))
                } else if fact_type == FactType::Recharge {
                    let fact = RechargeFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Recharge(fact))
                } else if fact_type == FactType::StunBreak {
                    let fact = StunBreakFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::StunBreak(fact))
                } else if fact_type == FactType::Time {
                    let fact = TimeFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Time(fact))
                } else if fact_type == FactType::Unblockable {
                    let fact = UnblockableFact::deserialize(v).map_err(de::Error::custom)?;
                    Ok(Fact::Unblockable(fact))
                } else {
                    panic!("the `type` field could not be deserialized to one of the listed FactType options listed");
                }
            }
            None => {
                let untyped_fact = UnTypedFact::deserialize(v).map_err(de::Error::custom)?;
                Ok(Fact::UnTyped(untyped_fact))
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
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

    // Sometimes there is no 'type' field on a fact, e.g. Nefarious Favor,
    // id: 40813. In this case there was only a 'text', 'icon', and 'percent'
    // field.
    // This can be handled by making Fact untagged, tagging all the variants'
    // ___Fact (e.g. BuffFact), with "type", and not tagging UnTypedFact.
    UnTyped(UnTypedFact),
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
            Fact::UnTyped(_) => FactType::UnTyped,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    // description field not always there, e.g. True Nature, id: 29393
    pub description: String,
    pub icon: Option<String>,
    pub chat_link: String,
    // not all skills have a 'type' field. For example; Binding Roots, id: 1279
    #[serde(rename = "type")]
    pub _type: Option<SkillType>,
    pub weapon_type: Option<WeaponType>,
    // not all skills have a 'professions' field. For example; Binding Roots, id: 1279
    pub professions: Option<Vec<Profession>>,
    // not all skills have a 'slot' field. For example; Binding Roots, id: 1279
    pub slot: Option<Slot>,
    pub traited_facts: Option<Vec<TraitedFact>>,
    pub categories: Option<Category>,
    pub attunement: Option<Attunement>,
    pub cost: Option<u16>,
    pub dual_wield: Option<WeaponType>,
    pub flip_skill: Option<SkillId>,
    pub initiative: Option<u8>,
    pub next_chain: Option<SkillId>,
    pub prev_chain: Option<SkillId>,
    pub transform_skills: Option<Vec<SkillId>>,
    pub bundle_skills: Option<Vec<SkillId>>,
    pub toolbelt_skill: Option<SkillId>,
    pub flags: Option<Vec<SkillFlag>>,
    // not all skills have a 'facts' field, for example Fire Attunement, id: 5492.
    pub facts: Option<Vec<Fact>>,
}

/// The following are attempts to make TraitedFact deserialize in a more
/// elegant way. Unfortunately none of them work.
///
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
// Try out the #[skip_serializing_none] serde attribute here.
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
