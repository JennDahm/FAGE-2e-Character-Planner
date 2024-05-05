#![allow(dead_code)]

use crate::{Advancement, Character, Class, DiceBasedHealthAdvancement};
use crate::{envoy, mage, rogue, warrior};
use crate::{AbilityDetermination, SelectName};

/// All Level 1 advancements.
#[derive(Debug, Clone, Default)]
pub struct Level1 {
    pub name: SelectName,

    pub abilities: AbilityDetermination,

    // TODO: Ancestry
    // TODO: Background?

    // In addition to class selection, this handles several additional things:
    //
    // * Base health.
    // * Weapon group training.
    // * Initial traits and specializations.
    // * Class-specific powers.
    // * Starting equipment.
    //
    // All of these are based on class selection.
    pub class: Level1ClassSelections,

    // Level1ClassSelections handles base health; this handles the health on top of that.
    pub health: DiceBasedHealthAdvancement,
}

/// The "Advancement" part of Level 1 just sets the player's level.
/// The real meat and potatoes are in the sub-advancements.
impl Advancement for Level1 {
    fn apply_self(&self, char: &mut Character) -> Result<bool, ()> {
        char.mechanical_properties.level = 1;
        Ok(true)
    }

    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement)) {
        f(&self.name);
        f(&self.abilities);
        f(&self.class);
        f(&self.health);
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        f(&mut self.name);
        f(&mut self.abilities);
        f(&mut self.class);
        f(&mut self.health);
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}


/// The player must choose their class, which comes with additional choices.
#[derive(Debug, Clone, Default)]
pub enum Level1ClassSelections {
    #[default]
    NoChoice,
    Envoy(envoy::Level1Selections),
    Mage(mage::Level1Selections),
    Rogue(rogue::Level1Selections),
    Warrior(warrior::Level1Selections),
}

/// The "Advancement" part of Class Selection just sets the character's class and base health.
/// Class-specific selections are sub-advancements.
impl Advancement for Level1ClassSelections {
    fn apply_self(&self, char: &mut Character) -> Result<bool, ()> {
        char.mechanical_properties.class = Some(match self {
            Self::NoChoice => return Ok(false),
            Self::Envoy(_) => Class::Envoy,
            Self::Mage(_) => Class::Mage,
            Self::Rogue(_) => Class::Rogue,
            Self::Warrior(_) => Class::Warrior,
        });
        // Set base health
        if let Some(class) = char.mechanical_properties.class {
            char.mechanical_properties.max_health = class.initial_base_health() as u16;
            char.status.health = class.initial_base_health() as u16;
        }
        Ok(true)
    }

    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement)) {
        match self {
            Self::NoChoice => (),
            Self::Envoy(sub) => f(sub),
            Self::Mage(sub) => f(sub),
            Self::Rogue(sub) => f(sub),
            Self::Warrior(sub) => f(sub),
        }
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        match self {
            Self::NoChoice => (),
            Self::Envoy(sub) => f(sub),
            Self::Mage(sub) => f(sub),
            Self::Rogue(sub) => f(sub),
            Self::Warrior(sub) => f(sub),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}
