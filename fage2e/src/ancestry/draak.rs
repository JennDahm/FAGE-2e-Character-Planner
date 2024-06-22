#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{into_generic_power_option, Ability, Advancement, AncestryBenefit, AncestryBenefitSelections, Character, DarkSightDetails, Focus, FocusLevel, LeafNodeAdvancement, PowerMechanics};

// -----------------------------------------------------------------------------
// LEVEL 1 SELECTIONS
// -----------------------------------------------------------------------------

/// The initial selections the user must make for this ancestry.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Level1Selections {
    /// Initial ability focus selection.
    pub ability_focus: Option<AbilityFocusSelection>,

    pub benefits: AncestryBenefitSelections<DraakBenefit>,
}

/// This top-level advancement only adds things the user doesn't have to select,
/// such as Dark Sight and available languages. Sub-advancements cover the user's
/// choices.
impl Advancement for Level1Selections {
    fn apply_self(&self, char: &mut Character) -> Result<bool, ()> {
        char.mechanical_properties.powers.dark_sight = Some(DarkSightDetails {});
        Ok(true)
    }

    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement)) {
        f(&self.ability_focus);
        f(&self.benefits);
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        f(&mut self.ability_focus);
        f(&mut self.benefits);
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}


/// The choices the user has for initial ability focuses.
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AbilityFocusSelection {
    Intimidation,
    SelfDiscipline,
}

impl std::fmt::Display for AbilityFocusSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Intimidation => Focus::StrengthIntimidation.fmt(f),
            Self::SelfDiscipline => Focus::WillpowerSelfDiscipline.fmt(f),
        }
    }
}

impl LeafNodeAdvancement for Option<AbilityFocusSelection> {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let focus = match self {
            Self::None => return Ok(false),
            Self::Some(AbilityFocusSelection::Intimidation) => Focus::StrengthIntimidation,
            Self::Some(AbilityFocusSelection::SelfDiscipline) => Focus::WillpowerSelfDiscipline,
        };
        if char.mechanical_properties.focuses.contains_key(&focus) {
            Ok(false)
        }
        else {
            char.mechanical_properties.focuses.insert(focus, FocusLevel::SingleFocus);
            Ok(true)
        }
    }
}

/// The Draak benefits the user can choose from.
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DraakBenefit {
    PlusOneConstitution,
    Armored,
    ConstitutionStamina,
    MagicalResistance(Option<MagicalResistanceFocusChoice>),
    PlusOneIntelligence,
    FlameBreath,
    IntelligenceResearch,
    PlusOneWillpower,
    PlusOneStrength,
}

impl std::fmt::Display for DraakBenefit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl AncestryBenefit for DraakBenefit {
    /// The display name of this benefit.
    fn display_name(&self) -> String {
        match self {
            Self::PlusOneConstitution => "+1 Constitution",
            Self::Armored => "Power: Armored",
            Self::ConstitutionStamina => "Constitution (Stamina)",
            Self::MagicalResistance(_) => "Power: Magical Resistance",
            Self::PlusOneIntelligence => "+1 Intelligence",
            Self::FlameBreath => "Ancestry Stunt: Flame Breath",
            Self::IntelligenceResearch => "Intelligence (Research)",
            Self::PlusOneWillpower => "+1 Willpower",
            Self::PlusOneStrength => "+1 Strength",
        }.to_owned()
    }

    /// For a given 2d6 roll, the corresponding benefit.
    fn from_roll(roll: u16) -> Result<Self, ()> {
        Ok(match roll {
            2 => Self::PlusOneConstitution,
            3..=4 => Self::Armored,
            5 => Self::ConstitutionStamina,
            6 => Self::MagicalResistance(None),
            7..=8 => Self::PlusOneIntelligence,
            9 => Self::FlameBreath,
            10 => Self::IntelligenceResearch,
            11 => Self::PlusOneWillpower,
            12 => Self::PlusOneStrength,
            _ => return Err(()),
        })
    }

    /// Whether this benefit counts as both choices when choosing manually.
    fn counts_as_two(&self) -> bool {
        match self {
            Self::PlusOneConstitution => true,
            Self::Armored => false,
            Self::ConstitutionStamina => false,
            Self::MagicalResistance(_) => false,
            Self::PlusOneIntelligence => true,
            Self::FlameBreath => false,
            Self::IntelligenceResearch => false,
            Self::PlusOneWillpower => true,
            Self::PlusOneStrength => true,
        }
    }

    /// Apply this benefit to the character.
    ///
    /// Returns whether the selection is complete.
    fn apply(&self, char: &mut Character) -> bool {
        match self {
            Self::PlusOneConstitution => {
                *char.mechanical_properties.abilities.get_mut(Ability::Constitution) += 1;
                true
            },
            Self::Armored => {
                char.mechanical_properties.powers.draak.armored = Some(ArmoredDetails {});
                true
            },
            Self::ConstitutionStamina => {
                char.mechanical_properties.focuses.insert(Focus::ConstitutionStamina, FocusLevel::SingleFocus);
                true
            },
            Self::MagicalResistance(None) => {
                false
            },
            Self::MagicalResistance(Some(focus_choice)) => {
                char.mechanical_properties.powers.draak.magical_resistance = Some(
                    MagicalResistanceDetails { focus_choice: *focus_choice }
                );
                true
            },
            Self::PlusOneIntelligence => {
                *char.mechanical_properties.abilities.get_mut(Ability::Intelligence) += 1;
                true
            },
            Self::FlameBreath => {
                // TODO
                true
            },
            Self::IntelligenceResearch => {
                char.mechanical_properties.focuses.insert(Focus::IntelligenceResearch, FocusLevel::SingleFocus);
                true
            },
            Self::PlusOneWillpower => {
                *char.mechanical_properties.abilities.get_mut(Ability::Willpower) += 1;
                true
            },
            Self::PlusOneStrength => {
                *char.mechanical_properties.abilities.get_mut(Ability::Strength) += 1;
                true
            },
        }
    }
}

// -----------------------------------------------------------------------------
// POWERS
// -----------------------------------------------------------------------------

/// Draak-specific powers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DraakPower {
    Armored,
    MagicalResistance,
}

impl std::fmt::Display for DraakPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Armored => write!(f, "Armored (Draak)"),
            Self::MagicalResistance => write!(f, "Magical Resistance (Draak)"),
        }
    }
}

/// Details about the various Draak powers a character might have.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DraakPowerDetails {
    pub armored: Option<ArmoredDetails>,
    pub magical_resistance: Option<MagicalResistanceDetails>,
}

impl DraakPowerDetails {
    /// Iterate over the powers selected by the user.
    pub fn iter(&self) -> impl Iterator<Item = &dyn PowerMechanics> {
        [
            into_generic_power_option(&self.armored),
            into_generic_power_option(&self.magical_resistance),
        ]
            .into_iter()
            .filter_map(|opt| opt)
    }

    /// Look up a power by ID.
    pub fn lookup(&self, power: DraakPower) -> Option<&dyn PowerMechanics> {
        match power {
            DraakPower::Armored => into_generic_power_option(&self.armored),
            DraakPower::MagicalResistance => into_generic_power_option(&self.magical_resistance),
        }
    }
}

/// Metadata about the Armored power.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArmoredDetails {
    // There aren't actually any choices here, so there's nothing to store.
}

impl PowerMechanics for ArmoredDetails {
    fn power(&self) -> crate::Power {
        DraakPower::Armored.into()
    }

    fn name(&self) -> String {
        "Armored (Draak)".to_owned()
    }

    fn description(&self) -> String {
        "+2 Armor Rating".to_owned()
    }

    fn armor_bonus(&self) -> Option<i8> {
        Some(2)
    }
}

/// Metadata about the Magical Resistance power.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MagicalResistanceDetails {
    /// The focus the player chose to use for magic resistance checks.
    pub focus_choice: MagicalResistanceFocusChoice,
}

impl PowerMechanics for MagicalResistanceDetails {
    fn power(&self) -> crate::Power {
        DraakPower::MagicalResistance.into()
    }

    fn name(&self) -> String {
        "Magical Resistance (Draak)".to_owned()
    }

    fn description(&self) -> String {
        format!("Use {} to resist or reduce the effects of a spell.", self.focus_choice.focus())
    }
}

/// Which focus the player chose for Magical Resistance.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MagicalResistanceFocusChoice {
    ConstitutionStamina,
    IntelligenceArcaneLore,
    WillpowerSelfDiscipline,
}

impl MagicalResistanceFocusChoice {
    /// The corresponding focus for this choice.
    pub fn focus(&self) -> Focus {
        match self {
            Self::ConstitutionStamina => Focus::ConstitutionStamina,
            Self::IntelligenceArcaneLore => Focus::IntelligenceArcaneLore,
            Self::WillpowerSelfDiscipline => Focus::WillpowerSelfDiscipline,
        }
    }
}
