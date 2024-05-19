#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{Advancement, Character, Focus, FocusLevel, LeafNodeAdvancement};

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
    // TODO: Wildfolk species

    // Initial ability focus selection.
    pub ability_focus: Option<AbilityFocusSelection>,

    // TODO: Wildfolk benefits
}

/// This top-level advancement only adds things the user doesn't have to select,
/// such as available languages. Sub-advancements cover the user's choices.
impl Advancement for Level1Selections {
    fn apply_self(&self, _: &mut Character) -> Result<bool, ()> {
        Ok(true)
    }

    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement)) {
        f(&self.ability_focus);
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        f(&mut self.ability_focus);
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}


/// The choices the user has for initial ability focuses.
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AbilityFocusSelection {
    Stamina,
    Seeing,
}

impl std::fmt::Display for AbilityFocusSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stamina => Focus::ConstitutionStamina.fmt(f),
            Self::Seeing => Focus::PerceptionSeeing.fmt(f),
        }
    }
}

impl LeafNodeAdvancement for Option<AbilityFocusSelection> {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let focus = match self {
            Self::None => return Ok(false),
            Self::Some(AbilityFocusSelection::Stamina) => Focus::ConstitutionStamina,
            Self::Some(AbilityFocusSelection::Seeing) => Focus::PerceptionSeeing,
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
