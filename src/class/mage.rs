#![allow(dead_code)]
//! Various details about the Mage class.

use crate::Ability;

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Accuracy, Ability::Intelligence, Ability::Perception, Ability::Willpower,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Communication, Ability::Constitution, Ability::Dexterity, Ability::Fighting, Ability::Strength,
];

pub static STARTING_HEALTH: u8 = 20;
