#![allow(dead_code)]
//! Abilities and ability focuses

use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Ability {
    Accuracy,
    Communication,
    Constitution,
    Dexterity,
    Fighting,
    Intelligence,
    Perception,
    Strength,
    Willpower,
}

impl Ability {
    /// The display name for this ability.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Accuracy => "Accuracy",
            Self::Communication => "Communication",
            Self::Constitution => "Constitution",
            Self::Dexterity => "Dexterity",
            Self::Fighting => "Fighting",
            Self::Intelligence => "Intelligence",
            Self::Perception => "Perception",
            Self::Strength => "Strength",
            Self::Willpower => "Willpower",
        }
    }

    /// An iterator over this ability's corresponding focuses.
    pub fn focuses(self) -> impl Iterator<Item = Focus> {
        Focus::iter().filter(move |f| f.ability() == self)
    }
}

impl std::fmt::Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Focus {
    // ACCURACY
    AccuracyArcaneBlast,
    AccuracyBlackPowder,
    AccuracyBows,
    AccuracyBrawling,
    AccuracyDueling,
    AccuracyGrenades,
    AccuracyLightBlades,
    AccuracyStaves,
    AccuracySlings,

    // COMMUNICATION
    CommunicationAnimalHandling,
    CommunicationBargaining,
    CommunicationDeception,
    CommunicationDisguise,
    CommunicationEtiquette,
    CommunicationGambling,
    CommunicationInvestigation,
    CommunicationLeadership,
    CommunicationPerformance,
    CommunicationPersuasion,
    CommunicationSeduction,

    // CONSTITUTION
    ConstitutionRowing,
    ConstitutionRunning,
    ConstitutionStamina,
    ConstitutionSwimming,
    ConstitutionTolerance,

    // DEXTERITY
    DexterityAcrobatics,
    DexterityCalligraphy,
    DexterityCrafting,
    DexterityInitiative,
    DexterityLegerdemain,
    DexterityLockPicking,
    DexterityRiding,
    DexteritySailing,
    DexterityStealth,
    DexterityTraps,

    // FIGHTING
    FightingAxes,
    FightingBludgeons,
    FightingHeavyBlades,
    FightingLances,
    FightingPolearms,
    FightingSpears,

    // INTELLIGENCE
    IntelligenceArcana,  // TODO: There are multiple variations on this, aren't there?
    IntelligenceArcaneLore,
    IntelligenceBrewing,
    IntelligenceCartography,
    IntelligenceCryptography,
    IntelligenceCulturalLore,
    IntelligenceEngineering,
    IntelligenceEvaluation,
    IntelligenceHealing,
    IntelligenceHeraldry,
    IntelligenceHistoricalLore,
    IntelligenceMilitaryLore,
    IntelligenceMusicalLore,
    IntelligenceNaturalLore,
    IntelligenceNavigation,
    IntelligenceReligiousLore,
    IntelligenceResearch,
    IntelligenceThievesLore,
    IntelligenceWriting,

    // PERCEPTION
    PerceptionEmpathy,
    PerceptionHearing,
    PerceptionSearching,
    PerceptionSeeing,
    PerceptionSmelling,
    PerceptionTasting,
    PerceptionTouching,
    PerceptionTracking,

    // STRENGTH
    StrengthClimbing,
    StrengthDriving,
    StrengthIntimidation,
    StrengthJumping,
    StrengthMight,
    StrengthSmithing,

    // WILLPOWER
    WillpowerCourage,
    WillpowerFaith,
    WillpowerMorale,
    WillpowerSelfDiscipline,
}

impl Focus {
    /// This focus's corresponding ability.
    pub fn ability(&self) -> Ability {
        match self {
            // ACCURACY
            Self::AccuracyArcaneBlast |
            Self::AccuracyBlackPowder |
            Self::AccuracyBows |
            Self::AccuracyBrawling |
            Self::AccuracyDueling |
            Self::AccuracyGrenades |
            Self::AccuracyLightBlades |
            Self::AccuracyStaves |
            Self::AccuracySlings => Ability::Accuracy,

            // COMMUNICATION
            Self::CommunicationAnimalHandling |
            Self::CommunicationBargaining |
            Self::CommunicationDeception |
            Self::CommunicationDisguise |
            Self::CommunicationEtiquette |
            Self::CommunicationGambling |
            Self::CommunicationInvestigation |
            Self::CommunicationLeadership |
            Self::CommunicationPerformance |
            Self::CommunicationPersuasion |
            Self::CommunicationSeduction => Ability::Communication,

            // CONSTITUTION
            Self::ConstitutionRowing |
            Self::ConstitutionRunning |
            Self::ConstitutionStamina |
            Self::ConstitutionSwimming |
            Self::ConstitutionTolerance => Ability::Constitution,

            // DEXTERITY
            Self::DexterityAcrobatics |
            Self::DexterityCalligraphy |
            Self::DexterityCrafting |
            Self::DexterityInitiative |
            Self::DexterityLegerdemain |
            Self::DexterityLockPicking |
            Self::DexterityRiding |
            Self::DexteritySailing |
            Self::DexterityStealth |
            Self::DexterityTraps => Ability::Dexterity,

            // FIGHTING
            Self::FightingAxes |
            Self::FightingBludgeons |
            Self::FightingHeavyBlades |
            Self::FightingLances |
            Self::FightingPolearms |
            Self::FightingSpears => Ability::Fighting,

            // INTELLIGENCE
            Self::IntelligenceArcana |
            Self::IntelligenceArcaneLore |
            Self::IntelligenceBrewing |
            Self::IntelligenceCartography |
            Self::IntelligenceCryptography |
            Self::IntelligenceCulturalLore |
            Self::IntelligenceEngineering |
            Self::IntelligenceEvaluation |
            Self::IntelligenceHealing |
            Self::IntelligenceHeraldry |
            Self::IntelligenceHistoricalLore |
            Self::IntelligenceMilitaryLore |
            Self::IntelligenceMusicalLore |
            Self::IntelligenceNaturalLore |
            Self::IntelligenceNavigation |
            Self::IntelligenceReligiousLore |
            Self::IntelligenceResearch |
            Self::IntelligenceThievesLore |
            Self::IntelligenceWriting => Ability::Intelligence,

            // PERCEPTION
            Self::PerceptionEmpathy |
            Self::PerceptionHearing |
            Self::PerceptionSearching |
            Self::PerceptionSeeing |
            Self::PerceptionSmelling |
            Self::PerceptionTasting |
            Self::PerceptionTouching |
            Self::PerceptionTracking => Ability::Perception,

            // STRENGTH
            Self::StrengthClimbing |
            Self::StrengthDriving |
            Self::StrengthIntimidation |
            Self::StrengthJumping |
            Self::StrengthMight |
            Self::StrengthSmithing => Ability::Strength,

            // WILLPOWER
            Self::WillpowerCourage |
            Self::WillpowerFaith |
            Self::WillpowerMorale |
            Self::WillpowerSelfDiscipline => Ability::Willpower,
        }
    }

    /// The base display name for this ability.
    pub fn base_name(&self) -> &'static str {
        match self {
            // ACCURACY
            Self::AccuracyArcaneBlast => "Arcane Blast",
            Self::AccuracyBlackPowder => "Black Powder",
            Self::AccuracyBows => "Bows",
            Self::AccuracyBrawling => "Brawling",
            Self::AccuracyDueling => "Dueling",
            Self::AccuracyGrenades => "Grenades",
            Self::AccuracyLightBlades => "Light Blades",
            Self::AccuracyStaves => "Staves",
            Self::AccuracySlings => "Slings",

            // COMMUNICATION
            Self::CommunicationAnimalHandling => "Animal Handling",
            Self::CommunicationBargaining => "Bargaining",
            Self::CommunicationDeception => "Deception",
            Self::CommunicationDisguise => "Disguise",
            Self::CommunicationEtiquette => "Etiquette",
            Self::CommunicationGambling => "Gambling",
            Self::CommunicationInvestigation => "Investigation",
            Self::CommunicationLeadership => "Leadership",
            Self::CommunicationPerformance => "Performance",
            Self::CommunicationPersuasion => "Persuasion",
            Self::CommunicationSeduction => "Seduction",

            // CONSTITUTION
            Self::ConstitutionRowing => "Rowing",
            Self::ConstitutionRunning => "Running",
            Self::ConstitutionStamina => "Stamina",
            Self::ConstitutionSwimming => "Swimming",
            Self::ConstitutionTolerance => "Tolerance",

            // DEXTERITY
            Self::DexterityAcrobatics => "Acrobatics",
            Self::DexterityCalligraphy => "Calligraphy",
            Self::DexterityCrafting => "Crafting",
            Self::DexterityInitiative => "Initiative",
            Self::DexterityLegerdemain => "Legerdemain",
            Self::DexterityLockPicking => "Lock Picking",
            Self::DexterityRiding => "Riding",
            Self::DexteritySailing => "Sailing",
            Self::DexterityStealth => "Stealth",
            Self::DexterityTraps => "Traps",

            // FIGHTING
            Self::FightingAxes => "Axes",
            Self::FightingBludgeons => "Bludgeons",
            Self::FightingHeavyBlades => "Heavy Blades",
            Self::FightingLances => "Lances",
            Self::FightingPolearms => "Polearms",
            Self::FightingSpears => "Spears",

            // INTELLIGENCE
            Self::IntelligenceArcana => "Arcana",
            Self::IntelligenceArcaneLore => "Arcane Lore",
            Self::IntelligenceBrewing => "Brewing",
            Self::IntelligenceCartography => "Cartography",
            Self::IntelligenceCryptography => "Cryptography",
            Self::IntelligenceCulturalLore => "Cultural Lore",
            Self::IntelligenceEngineering => "Engineering",
            Self::IntelligenceEvaluation => "Evaluation",
            Self::IntelligenceHealing => "Healing",
            Self::IntelligenceHeraldry => "Heraldry",
            Self::IntelligenceHistoricalLore => "Historical Lore",
            Self::IntelligenceMilitaryLore => "Military Lore",
            Self::IntelligenceMusicalLore => "Musical Lore",
            Self::IntelligenceNaturalLore => "Natural Lore",
            Self::IntelligenceNavigation => "Navigation",
            Self::IntelligenceReligiousLore => "Religious Lore",
            Self::IntelligenceResearch => "Research",
            Self::IntelligenceThievesLore => "Thieves' Lore",
            Self::IntelligenceWriting => "Writing",

            // PERCEPTION
            Self::PerceptionEmpathy => "Empathy",
            Self::PerceptionHearing => "Hearing",
            Self::PerceptionSearching => "Searching",
            Self::PerceptionSeeing => "Seeing",
            Self::PerceptionSmelling => "Smelling",
            Self::PerceptionTasting => "Tasting",
            Self::PerceptionTouching => "Touching",
            Self::PerceptionTracking => "Tracking",

            // STRENGTH
            Self::StrengthClimbing => "Climbing",
            Self::StrengthDriving => "Driving",
            Self::StrengthIntimidation => "Intimidation",
            Self::StrengthJumping => "Jumping",
            Self::StrengthMight => "Might",
            Self::StrengthSmithing => "Smithing",

            // WILLPOWER
            Self::WillpowerCourage => "Courage",
            Self::WillpowerFaith => "Faith",
            Self::WillpowerMorale => "Morale",
            Self::WillpowerSelfDiscipline => "Self-Discipline",
        }
    }

    /// The full display name for this ability (e.g. "Willpower (Courage)").
    pub fn name(&self) -> String {
        format!("{} ({})", self.ability().name(), self.base_name())
    }
}

impl std::fmt::Display for Focus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// How much a character has invested in a particular focus.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FocusLevel {
    /// No investment at all.
    None,

    /// One level of investment.
    SingleFocus,

    /// Two levels of investment.
    DoubleFocus,
}

/// An individual ability score, keeping track of partial advancements.
///
/// This class supports adding advancements directly using + or -, and this handles
/// partial advancements for you.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AbilityScore {
    pub score: i8,
    pub partial: i8,
}

impl std::fmt::Display for AbilityScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.score)?;
        for _ in 1..=self.partial {
            write!(f, "+")?;
        }
        std::fmt::Result::Ok(())
    }
}

impl AbilityScore {
    /// Accumulate partials into whole scores where possible.
    ///
    /// How many advancements you need to invest in a score to increase it varies by level.
    /// The "Ability Advancement Table" in Chapter 1/Classes/Level Advancement defines these
    /// levels.
    fn settle_partials(&mut self) {
        // Handle negative partials (removing ability scores).
        while self.partial < 0 {
            self.partial += match self.score {
                9.. => 3,
                6..=8 => 2,
                ..=5 => 1,
            };
            self.score -= 1;
        }
        // Handle positive partials (adding ability scores).
        while self.partial > 0 {
            let threshold = match self.score + 1 {
                9.. => 3,
                6..=8 => 2,
                ..=5 => 1,
            };
            if self.partial < threshold {
                break;
            }
            self.partial -= threshold;
            self.score += 1;
        }
    }
}

impl std::ops::Add<i8> for AbilityScore {
    type Output = Self;

    fn add(mut self, rhs: i8) -> Self::Output {
        self.partial += rhs;
        self.settle_partials();
        self
    }
}

impl std::ops::AddAssign<i8> for AbilityScore {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<i8> for AbilityScore {
    type Output = Self;

    fn sub(mut self, rhs: i8) -> Self::Output {
        self.partial -= rhs;
        self.settle_partials();
        self
    }
}

impl std::ops::SubAssign<i8> for AbilityScore {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

/// A container for ability scores.
#[derive(Debug, Copy, Clone)]
pub struct AbilityScores {
    accuracy: AbilityScore,
    communication: AbilityScore,
    constitution: AbilityScore,
    dexterity: AbilityScore,
    fighting: AbilityScore,
    intelligence: AbilityScore,
    perception: AbilityScore,
    strength: AbilityScore,
    willpower: AbilityScore,
}

impl AbilityScores {
    pub fn new() -> AbilityScores {
        AbilityScores {
            accuracy: AbilityScore { score: 0, partial: 0 },
            communication: AbilityScore { score: 0, partial: 0 },
            constitution: AbilityScore { score: 0, partial: 0 },
            dexterity: AbilityScore { score: 0, partial: 0 },
            fighting: AbilityScore { score: 0, partial: 0 },
            intelligence: AbilityScore { score: 0, partial: 0 },
            perception: AbilityScore { score: 0, partial: 0 },
            strength: AbilityScore { score: 0, partial: 0 },
            willpower: AbilityScore { score: 0, partial: 0 },
        }
    }

    /// Get the specified ability score.
    pub fn get(&self, ability: Ability) -> AbilityScore {
        match ability {
            Ability::Accuracy => self.accuracy,
            Ability::Communication => self.communication,
            Ability::Constitution => self.constitution,
            Ability::Dexterity => self.dexterity,
            Ability::Fighting => self.fighting,
            Ability::Intelligence => self.intelligence,
            Ability::Perception => self.perception,
            Ability::Strength => self.strength,
            Ability::Willpower => self.willpower,
        }
    }

    /// Get a mutable reference to the specified ability score.
    pub fn get_mut(&mut self, ability: Ability) -> &mut AbilityScore {
        match ability {
            Ability::Accuracy => &mut self.accuracy,
            Ability::Communication => &mut self.communication,
            Ability::Constitution => &mut self.constitution,
            Ability::Dexterity => &mut self.dexterity,
            Ability::Fighting => &mut self.fighting,
            Ability::Intelligence => &mut self.intelligence,
            Ability::Perception => &mut self.perception,
            Ability::Strength => &mut self.strength,
            Ability::Willpower => &mut self.willpower,
        }
    }

    /// Set the specified ability score.
    pub fn set(&mut self, ability: Ability, score: AbilityScore) {
        *self.get_mut(ability) = score;
    }
}
