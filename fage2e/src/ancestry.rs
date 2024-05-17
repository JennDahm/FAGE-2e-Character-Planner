#![allow(dead_code)]

use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod draak;
pub mod dwarf;
pub mod elf;
pub mod gnome;
pub mod goblin;
pub mod halfling;
pub mod human;
pub mod orc;
pub mod wildfolk;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Ancestry {
    Draak,
    Dwarf,
    Elf,
    Gnome,
    Goblin,
    Halfling,
    Human,
    Orc,
    Wildfolk(Option<wildfolk::Species>),
}

impl Ancestry {
    /// The display name for this ancestry.
    pub fn name(&self) -> String {
        match self {
            Self::Draak => "Draak".to_owned(),
            Self::Dwarf => "Dwarf".to_owned(),
            Self::Elf => "Elf".to_owned(),
            Self::Gnome => "Gnome".to_owned(),
            Self::Goblin => "Goblin".to_owned(),
            Self::Halfling => "Halfling".to_owned(),
            Self::Human => "Human".to_owned(),
            Self::Orc => "Orc".to_owned(),
            Self::Wildfolk(None) => "Wildfolk".to_owned(),
            Self::Wildfolk(Some(species)) => format!("Wildfolk ({})", species.name()),
        }
    }

    /// The initial base speed for this ancestry.
    pub fn initial_base_speed(&self) -> u8 {
        match self {
            Self::Draak => 10,
            Self::Dwarf => 8,
            Self::Elf => 12,
            Self::Gnome => 8,
            Self::Goblin => 10,
            Self::Halfling => 8,
            Self::Human => 10,
            Self::Orc => 10,
            Self::Wildfolk(_) => 10,
        }
    }
}

impl std::fmt::Display for Ancestry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
