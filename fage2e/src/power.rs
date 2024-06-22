#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::draak;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Power {
    // General powers
    DarkSight,

    // Talents and Specializations
    // Talent(),  // TODO
    // Specialization(), // TODO

    // Class-based powers
    // Envoy(),    // TODO
    // Mage(),     // TODO
    // Rogue(),    // TODO
    // Warrior(),  // TODO

    // Ancestry-based powers
    Draak(draak::DraakPower),
    // Dwarf(),     // TODO: Stout
    // Elf ancestry has no special powers.
    // Gnome(),     // TODO: Animal Speech
    // Goblin(),    // TODO: Swift
    // Halfling(),  // TODO: Steady
    // Human(),     // TODO: Adaptable Focus
    // Orc(),       // TODO: Tough
    // Wildfolk(),  // TODO: Glide, Natural Weapon
}

impl From<draak::DraakPower> for Power {
    fn from(value: draak::DraakPower) -> Self {
        Power::Draak(value)
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DarkSight => write!(f, "Dark Sight"),
            Self::Draak(power) => power.fmt(f),
        }
    }
}

// TODO: From implementations for EnvoyPower, MagePower, etc.

/// The mechanics for a given power.
pub trait PowerMechanics {
    /// What power this object references.
    fn power(&self) -> Power;

    /// A display name for the power.
    fn name(&self) -> String;

    /// A description of the power.
    fn description(&self) -> String;

    /// The armor bonus this power provides, if any.
    fn armor_bonus(&self) -> Option<i8> {
        None
    }

    // TODO
}

/// Maps a reference to an optional power mechanics object into an optional
/// generic reference to the PowerMechanics trait.
pub fn into_generic_power_option<T: PowerMechanics>(value: &Option<T>) -> Option<&dyn PowerMechanics> {
    match value {
        Some(v) => Some(v as &dyn PowerMechanics),
        None => None,
    }
}


/// Details about the various powers a character might have.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerDetails {
    pub dark_sight: Option<DarkSightDetails>,

    pub draak: draak::DraakPowerDetails,
}

impl PowerDetails {
    /// Iterate over the powers selected by the user.
    pub fn iter(&self) -> impl Iterator<Item = &dyn PowerMechanics> {
        [
            into_generic_power_option(&self.dark_sight)
        ]
            .into_iter()
            .filter_map(|opt| opt)
            .chain(self.draak.iter())
    }
}

/// Metadata about the Dark Sight power.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DarkSightDetails {
    // There aren't actually any choices here, so there's nothing to store.
}

impl PowerMechanics for DarkSightDetails {
    fn power(&self) -> Power {
        Power::DarkSight
    }

    fn name(&self) -> String {
        "Dark Sight".to_owned()
    }

    fn description(&self) -> String {
        "You can see up to 20 yards in darkness without a light source.".to_owned()
    }
}
