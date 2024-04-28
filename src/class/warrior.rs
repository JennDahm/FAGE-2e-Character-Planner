#![allow(dead_code)]
//! Various details about the Warrior class.

use crate::Ability;

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Constitution, Ability::Dexterity, Ability::Fighting, Ability::Strength,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Accuracy, Ability::Communication, Ability::Intelligence, Ability::Perception, Ability::Willpower,
];

pub static STARTING_HEALTH: u8 = 30;
