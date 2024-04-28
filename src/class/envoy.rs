#![allow(dead_code)]
//! Various details about the Envoy class.

use crate::Ability;

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Communication, Ability::Fighting, Ability::Intelligence, Ability::Willpower,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Accuracy, Ability::Constitution, Ability::Dexterity, Ability::Perception, Ability::Strength,
];

pub static STARTING_HEALTH: u8 = 25;
