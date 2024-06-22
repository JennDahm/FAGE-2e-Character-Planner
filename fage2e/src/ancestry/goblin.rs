#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{Advancement, Character, DarkSightDetails, Focus, FocusLevel, LeafNodeAdvancement};

/// The initial selections the user must make for this ancestry.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Level1Selections {
    /// Initial ability focus selection.
    pub ability_focus: Option<AbilityFocusSelection>,

    // TODO: Goblin benefits
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
    Investigation,
    Stealth,
}

impl std::fmt::Display for AbilityFocusSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Investigation => Focus::CommunicationInvestigation.fmt(f),
            Self::Stealth => Focus::DexterityStealth.fmt(f),
        }
    }
}

impl LeafNodeAdvancement for Option<AbilityFocusSelection> {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let focus = match self {
            Self::None => return Ok(false),
            Self::Some(AbilityFocusSelection::Investigation) => Focus::CommunicationInvestigation,
            Self::Some(AbilityFocusSelection::Stealth) => Focus::DexterityStealth,
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
