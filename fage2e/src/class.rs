#![allow(dead_code)]

use strum::EnumIter;

use crate::Ability;

pub mod envoy;
pub mod mage;
pub mod rogue;
pub mod warrior;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Class {
    Envoy,
    Mage,
    Rogue,
    Warrior,
}

impl Class {
    /// The display name for this class.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Envoy => "Envoy",
            Self::Mage => "Mage",
            Self::Rogue => "Rogue",
            Self::Warrior => "Warrior",
        }
    }

    /// This class's primary abilities.
    pub fn primary_abilities(&self) -> &'static [Ability] {
        match self {
            Self::Envoy => &envoy::PRIMARY_ABILITIES,
            Self::Mage => &mage::PRIMARY_ABILITIES,
            Self::Rogue => &rogue::PRIMARY_ABILITIES,
            Self::Warrior => &warrior::PRIMARY_ABILITIES,
        }
    }

    /// This class's secondary abilities.
    pub fn secondary_abilities(&self) -> &'static [Ability] {
        match self {
            Self::Envoy => &envoy::SECONDARY_ABILITIES,
            Self::Mage => &mage::SECONDARY_ABILITIES,
            Self::Rogue => &rogue::SECONDARY_ABILITIES,
            Self::Warrior => &warrior::SECONDARY_ABILITIES,
        }
    }

    /// This class's initial base health. The character's base health will be this + Constitution + d6.
    pub fn initial_base_health(&self) -> u8 {
        match self {
            Self::Envoy => envoy::STARTING_HEALTH,
            Self::Mage => mage::STARTING_HEALTH,
            Self::Rogue => rogue::STARTING_HEALTH,
            Self::Warrior => warrior::STARTING_HEALTH,
        }
    }
}
