#![allow(dead_code)]
//! All information about a character, including their scores, stats, and equipment.

use std::collections::{HashMap, HashSet};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    Ability, AbilityScores, AdditiveModifier, Ancestry, BaseValue, Class, Focus, FocusLevel, ModifierSet, ModifierSource, Value, Weapon, WeaponGroup
};

/// Non-mechanical properties of a character.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CharacterFlavor {
    pub name: String,

    // TODO: Apparently, Envoy gets two of each of these.
    pub background: Option<String>,  // TODO: Enum? Variant?
    pub social_class: Option<String>,  // TODO: Enum? Variant?

    pub backstory: Option<String>,
}

/// Mechanical properties of a character.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CharacterMechanicalProperties {
    /// The character's level. Valid values are 1-20.
    pub level: u8,

    /// The character's ancestry, if they've selected one.
    pub ancestry: Option<Ancestry>,

    /// The character's class, if they've selected one.
    pub class: Option<Class>,

    /// The character's ability scores.
    pub abilities: AbilityScores,

    /// The character's focuses.
    pub focuses: HashMap<Focus, FocusLevel>,

    /// What weapons the character is trained in.
    pub weapon_training: HashSet<WeaponGroup>,

    /// The character's base armor rating.
    pub base_armor: u8,

    /// The health advancements the character has earned over the levels.
    pub health_advancements: Vec<AdditiveModifier>,

    /// The defense advancements the character has earned over the levels.
    pub defense_advancements: Vec<AdditiveModifier>,

    // TODO: Talents
    // TODO: Specializations
    // TODO: Miscellaneous powers
    // TODO: MP
    // TODO: Spells
}

/// The character's equipment.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CharacterEquipment {
    /// The character's weapon cache.
    pub weapons: Vec<Weapon>,

    // TODO: armor
    // TODO: shields
}

/// On-going stats about a character.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
                ancestry: None,
                abilities: AbilityScores::new(),
                focuses: HashMap::new(),
                weapon_training: HashSet::new(),
                base_armor: 0,
                health_advancements: Vec::new(),
                defense_advancements: Vec::new(),
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

    /// The character's maximum health.
    pub fn max_health(&self) -> Value {
        let modifiers = ModifierSet {
            override_: None,
            additive: self.mechanical_properties.health_advancements.clone(),
        };
        if let Some(class) = self.mechanical_properties.class {
            Value {
                base: BaseValue {
                    value: class.initial_base_health() as i16,
                    source: ModifierSource::Class(class),
                },
                modifiers
            }
        }
        else {
            Value {
                base: BaseValue { value: 0, source: ModifierSource::Core },
                modifiers
            }
        }
    }

    /// The modifiers that go into the character's move speed.
    pub fn speed_yards(&self) -> Value {
        // Per Chapter 1, Step 7 (Defense and Speed), speed is:
        //   base speed (from ancestry) + dexterity - armor penalty
        // TODO: Armor penalty
        let modifiers = ModifierSet {
            override_: None,
            additive: vec![
                AdditiveModifier {
                    value: self.mechanical_properties.abilities.get(Ability::Dexterity).score,
                    source: ModifierSource::Ability(Ability::Dexterity),
                },
            ],
        };
        if let Some(ancestry) = self.mechanical_properties.ancestry {
            Value {
                base: BaseValue {
                    value: ancestry.initial_base_speed() as i16,
                    source: ModifierSource::Ancestry(ancestry),
                },
                modifiers
            }
        }
        else {
            Value {
                base: BaseValue { value: 0, source: ModifierSource::Core },
                modifiers
            }
        }
    }

    /// The character's move speed in yards.
    pub fn move_speed_yards(&self) -> u16 {
        let speed = self.speed_yards().final_value();

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

    /// The character's defense.
    pub fn defense(&self) -> Value {
        // Per Chapter 1, Step 7 (Defense and Speed), defense is:
        //   base defense + dexterity + shield bonus
        // TODO: Shield bonus
        let mut modifiers = ModifierSet {
            override_: None,
            additive: self.mechanical_properties.defense_advancements.clone(),
        };
        modifiers.additive.push(
            AdditiveModifier {
                value: self.mechanical_properties.abilities.get(Ability::Dexterity).score,
                source: ModifierSource::Ability(Ability::Dexterity),
            }
        );
        Value {
            base: BaseValue { value: 10, source: ModifierSource::Core },
            modifiers
        }
    }

    /// The character's armor rating.
    pub fn armor(&self) -> Value {
        Value {
            base: BaseValue { value: 0, source: ModifierSource::Core },
            modifiers: ModifierSet {
                override_: None,
                additive: Vec::new(),
            },
        }
    }
}
