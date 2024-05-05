#![allow(dead_code)]
//! All information about a character, including their scores, stats, and equipment.

use std::collections::{HashMap, HashSet};

use crate::{
    Ability,
    AbilityScores,
    Class,
    Focus,
    FocusLevel,
    Weapon,
    WeaponGroup,
    ModifierSource,
    AdditiveModifier,
    ModifierSet,
    Value,
};

/// Non-mechanical properties of a character.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterFlavor {
    pub name: String,

    // TODO: Apparently, Envoy gets two of each of these.
    pub background: Option<String>,  // TODO: Enum? Variant?
    pub social_class: Option<String>,  // TODO: Enum? Variant?

    pub backstory: Option<String>,
}

/// Mechanical properties of a character.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterMechanicalProperties {
    /// The character's level. Valid values are 1-20.
    pub level: u8,

    // TODO: race

    /// The character's class, if they've selected one.
    pub class: Option<Class>,

    /// The character's ability scores.
    pub abilities: AbilityScores,

    /// The character's focuses.
    pub focuses: HashMap<Focus, FocusLevel>,

    /// What weapons the character is trained in.
    pub weapon_training: HashSet<WeaponGroup>,

    /// The character's base speed in yards.
    pub base_speed_yards: u8,

    /// The character's base defense.
    pub base_defense: u8,

    /// The character's base armor rating.
    pub base_armor: u8,

    /// The character's max health.
    pub max_health: u16,

    // TODO: Talents
    // TODO: Specializations
    // TODO: Miscellaneous powers
    // TODO: MP
    // TODO: Spells
}

/// The character's equipment.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterEquipment {
    /// The character's weapon cache.
    pub weapons: Vec<Weapon>,

    // TODO: armor
    // TODO: shields
}

/// On-going stats about a character.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterStatus {
    /// The character's current number of experience points.
    pub exp: u32,

    /// The character's current health.
    pub health: u16,

    // TODO: Conditions

    // TODO: left/right hand equipment
    // TODO: ammo
}

/// A full character description.
#[derive(Debug, Clone, PartialEq)]
pub struct Character {
    pub flavor: CharacterFlavor,
    pub mechanical_properties: CharacterMechanicalProperties,
    pub equipment: CharacterEquipment,
    pub status: CharacterStatus,
}

impl Character {
    /// Starts the character building process.
    pub fn new() -> Character {
        Character {
            flavor: CharacterFlavor {
                name: String::new(),
                background: None,
                social_class: None,
                backstory: None,
            },
            mechanical_properties: CharacterMechanicalProperties {
                level: 0,
                class: None,
                abilities: AbilityScores::new(),
                focuses: HashMap::new(),
                weapon_training: HashSet::new(),
                base_speed_yards: 0,
                base_defense: 10,
                base_armor: 0,
                max_health: 0,
            },
            equipment: CharacterEquipment {
                weapons: Vec::new(),
            },
            status: CharacterStatus {
                exp: 0,
                health: 0,
            },
        }
    }

    /// The modifiers that go into the character's move speed.
    pub fn speed_value(&self) -> Value {
        // Per Chapter 1, Step 7 (Defense and Speed), speed is:
        //  base speed + dexterity - armor penalty
        // TODO: Armor penalty
        Value {
            base: self.mechanical_properties.base_speed_yards as i16,
            modifiers: ModifierSet {
                override_: None,
                additive: vec![
                    AdditiveModifier {
                        value: self.mechanical_properties.abilities.get(Ability::Dexterity).score,
                        source: ModifierSource::Ability(Ability::Dexterity),
                    },
                ],
            },
        }
    }

    /// The character's move speed in yards.
    pub fn move_speed_yards(&self) -> u16 {
        let speed = self.speed_value().final_value();

        // Speed can't be less than 0.
        if speed < 0 { 0u16 } else { speed as u16 }
    }

    /// The character's run speed in yards.
    pub fn run_speed_yards(&self) -> u16 {
        // Per Chapter 2/Combat/Movement, run is double your move speed.
        self.move_speed_yards() * 2
    }

    /// The character's charge speed in yards.
    pub fn charge_speed_yards(&self) -> u16 {
        // Per Chapter 2/Combat/Movement, charge is half your move speed rounded up.
        (self.move_speed_yards() + 1) / 2
    }

    /// The modifiers that go into the character's defense.
    pub fn defense_value(&self) -> Value {
        // Per Chapter 1, Step 7 (Defense and Speed), defense is:
        //  base defense + dexterity + shield bonus
        // TODO: Shield bonus
        Value {
            base: self.mechanical_properties.base_defense as i16,
            modifiers: ModifierSet {
                override_: None,
                additive: vec![
                    AdditiveModifier {
                        value: self.mechanical_properties.abilities.get(Ability::Dexterity).score,
                        source: ModifierSource::Ability(Ability::Dexterity),
                    },
                ],
            },
        }
    }

    /// The character's defense.
    pub fn defense(&self) -> u8 {
        let defense = self.defense_value().final_value();

        // Defense can't be less than 0.
        if defense < 0 { 0u8 } else { defense as u8 }
    }
}
