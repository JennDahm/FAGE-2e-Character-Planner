#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{draak, dwarf, elf, gnome, goblin, halfling, human, orc, wildfolk, Advancement, Ancestry, Character, Class, DiceBasedHealthAdvancement};
use crate::{envoy, mage, rogue, warrior};
use crate::{AbilityDetermination, SelectName};

/// All Level 1 advancements.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    pub ancestry: Level1AncestrySelections,

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
        f(&self.ancestry);
        f(&self.health);
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        f(&mut self.name);
        f(&mut self.abilities);
        f(&mut self.class);
        f(&mut self.ancestry);
        f(&mut self.health);
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}


/// The player must choose their class, which comes with additional choices.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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


/// The player must choose their ancestry, which comes with additional choices.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Level1AncestrySelections {
    #[default]
    NoChoice,
    Draak(draak::Level1Selections),
    Dwarf(dwarf::Level1Selections),
    Elf(elf::Level1Selections),
    Gnome(gnome::Level1Selections),
    Goblin(goblin::Level1Selections),
    Halfling(halfling::Level1Selections),
    Human(human::Level1Selections),
    Orc(orc::Level1Selections),
    Wildfolk(wildfolk::Level1Selections),
}

/// The "Advancement" part of Ancestry Selection just sets the character's ancestry and base speed.
/// Ancestry-specific selections are sub-advancements.
impl Advancement for Level1AncestrySelections {
    fn apply_self(&self, char: &mut Character) -> Result<bool, ()> {
        char.mechanical_properties.ancestry = Some(match self {
            Self::NoChoice => return Ok(false),
            Self::Draak(_) => Ancestry::Draak,
            Self::Dwarf(_) => Ancestry::Dwarf,
            Self::Elf(_) => Ancestry::Elf,
            Self::Gnome(_) => Ancestry::Gnome,
            Self::Goblin(_) => Ancestry::Goblin,
            Self::Halfling(_) => Ancestry::Halfling,
            Self::Human(_) => Ancestry::Human,
            Self::Orc(_) => Ancestry::Orc,
            Self::Wildfolk(_) => Ancestry::Wildfolk(None),
        });
        // Set base speed
        if let Some(ancestry) = char.mechanical_properties.ancestry {
            char.mechanical_properties.base_speed_yards = ancestry.initial_base_speed();
        }
        Ok(true)
    }

    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement)) {
        match self {
            Self::NoChoice => (),
            Self::Draak(sub) => f(sub),
            Self::Dwarf(sub) => f(sub),
            Self::Elf(sub) => f(sub),
            Self::Gnome(sub) => f(sub),
            Self::Goblin(sub) => f(sub),
            Self::Halfling(sub) => f(sub),
            Self::Human(sub) => f(sub),
            Self::Orc(sub) => f(sub),
            Self::Wildfolk(sub) => f(sub),
        }
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        match self {
            Self::NoChoice => (),
            Self::Draak(sub) => f(sub),
            Self::Dwarf(sub) => f(sub),
            Self::Elf(sub) => f(sub),
            Self::Gnome(sub) => f(sub),
            Self::Goblin(sub) => f(sub),
            Self::Halfling(sub) => f(sub),
            Self::Human(sub) => f(sub),
            Self::Orc(sub) => f(sub),
            Self::Wildfolk(sub) => f(sub),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}
