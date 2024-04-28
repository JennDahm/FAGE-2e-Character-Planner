#![allow(dead_code)]
//! Various details about the Rogue class.

use crate::Ability;

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Accuracy, Ability::Communication, Ability::Dexterity, Ability::Perception,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Constitution, Ability::Fighting, Ability::Intelligence, Ability::Strength, Ability::Willpower,
];

pub static STARTING_HEALTH: u8 = 25;
