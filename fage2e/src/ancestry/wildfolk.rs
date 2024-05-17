#![allow(dead_code)]

use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Advancement, Character};

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Species {
    Avian,
    Canine,
    Vulpine,
    Feline,
    Herpestidae,
    Rodent,
    Leporidae,
    Ungulate,
    Ursine,
}

impl Species {
    /// The display name for this species.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Avian => "Avian",
            Self::Canine => "Canine",
            Self::Vulpine => "Vulpine",
            Self::Feline => "Feline",
            Self::Herpestidae => "Herpestidae",
            Self::Rodent => "Rodent",
            Self::Leporidae => "Leporidae",
            Self::Ungulate => "Ungulate",
            Self::Ursine => "Ursine",
        }
    }
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The initial selections the user must make for this ancestry.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Level1Selections {
    // TODO
}

/// This top-level advancement doesn't itself have any logic, but it has sub-advancements.
impl Advancement for Level1Selections {
    fn apply_self(&self, _: &mut Character) -> Result<bool, ()> {
        Ok(true)
    }

    fn foreach(&self, _: &mut dyn FnMut(&dyn Advancement)) {
        // TODO
    }

    fn foreach_mut(&mut self, _: &mut dyn FnMut(&mut dyn Advancement)) {
        // TODO
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}

