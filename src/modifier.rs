#![allow(dead_code, unused_imports)]
//! Tools for keeping track of modifiers and their sources.
//!
//! This is especially important for explaining to users where their numbers are coming from.

use super::{Ability, Focus, Dice, DiceWithMod};

/// The source of a modifier or override.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ModifierSource {
    Ability(Ability),
    Focus(Focus),
}

/// A simple additive modifier and its source.
#[derive(Debug, Copy, Clone)]
pub struct AdditiveModifier {
    /// The value of the modifier to add.
    pub value: i8,

    /// The modifier's source.
    pub source: ModifierSource,
}

/// A set of modifiers to a dice roll or simple value and their sources.
#[derive(Debug, Clone)]
pub struct ModifierSet<T: std::fmt::Debug + Clone> {
    /// If specified, this overrides the base value/roll.
    pub override_: Option<T>,

    /// A list of additive modifiers on top of the base value/roll.
    pub additive: Vec<AdditiveModifier>,

    // TODO: Optional divider on final result?
}

/// A value with modifiers.
#[derive(Debug, Clone)]
pub struct Value {
    /// The base value.
    pub base: i16,

    /// The modifiers.
    pub modifiers: ModifierSet<i16>,
}

impl Value {
    /// Calculate the final value post modifiers.
    pub fn final_value(&self) -> i16 {
        let mut val = match self.modifiers.override_ {
            Some(v) => v,
            None => self.base,
        };
        for additive in &self.modifiers.additive {
            val += additive.value as i16;
        }
        val
    }
}

/// A dice roll with modifiers.
#[derive(Debug, Clone)]
pub struct DiceRoll {
    /// The base dice roll and modifier.
    pub base: DiceWithMod,

    /// The modifiers.
    pub modifiers: ModifierSet<DiceWithMod>,
}

impl DiceRoll {
    /// Determine what dice to roll.
    pub fn dice(&self) -> Dice {
        match self.modifiers.override_ {
            Some(v) => v.dice,
            None => self.base.dice,
        }
    }

    /// Calculate the final value post-modifiers. Dice roll handled separately.
    pub fn final_value(&self, dice_sum: i16) -> i16 {
        let mut val = dice_sum + match self.modifiers.override_ {
            Some(v) => v.modifier as i16,
            None => self.base.modifier as i16,
        };
        for additive in &self.modifiers.additive {
            val += additive.value as i16;
        }
        val
    }
}
