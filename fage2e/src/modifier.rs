#![allow(dead_code, unused_imports)]
//! Tools for keeping track of modifiers and their sources.
//!
//! This is especially important for explaining to users where their numbers are coming from.

use super::{Ability, Ancestry, Class, Focus, Dice, DiceWithMod, Power};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The source of a modifier or override.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ModifierSource {
    Ability(Ability),
    Ancestry(Ancestry),
    Class(Class),
    Core,
    Focus(Focus),
    Level(u8),
    Power(Power),
}

impl std::fmt::Display for ModifierSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ability(ability) => ability.fmt(f),
            Self::Ancestry(ancestry) => ancestry.fmt(f),
            Self::Class(class) => class.fmt(f),
            Self::Core => write!(f, "Core"),
            Self::Focus(focus) => focus.fmt(f),
            Self::Level(level) => write!(f, "Level {}", level),
            Self::Power(power) => power.fmt(f),
        }
    }
}

/// The base value that modifiers apply to.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseValue<T: std::fmt::Debug + Clone> {
    /// The base value.
    pub value: T,

    /// The source of this value.
    pub source: ModifierSource,
}

/// A simple additive modifier and its source.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AdditiveModifier {
    /// The value of the modifier to add.
    pub value: i8,

    /// The modifier's source.
    pub source: ModifierSource,
}

/// A set of modifiers to a dice roll or simple value and their sources.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModifierSet<T: std::fmt::Debug + Clone> {
    /// If specified, this overrides the base value/roll.
    /// TODO: Also specify override source.
    pub override_: Option<BaseValue<T>>,

    /// A list of additive modifiers on top of the base value/roll.
    pub additive: Vec<AdditiveModifier>,

    // TODO: Optional divider on final result?
}

/// A value with modifiers.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Value {
    /// The base value.
    pub base: BaseValue<i16>,

    /// The modifiers.
    pub modifiers: ModifierSet<i16>,
}

impl Value {
    /// Calculate the final value post modifiers.
    pub fn final_value(&self) -> i16 {
        let mut val = match &self.modifiers.override_ {
            Some(v) => v.value,
            None => self.base.value,
        };
        for additive in &self.modifiers.additive {
            val += additive.value as i16;
        }
        val
    }
}

/// A dice roll with modifiers.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DiceRoll {
    /// The base dice roll and modifier.
    pub base: BaseValue<DiceWithMod>,

    /// The modifiers.
    pub modifiers: ModifierSet<DiceWithMod>,
}

impl DiceRoll {
    /// Determine what dice to roll.
    pub fn dice(&self) -> Dice {
        match &self.modifiers.override_ {
            Some(v) => v.value.dice,
            None => self.base.value.dice,
        }
    }

    /// Calculate the final value post-modifiers. Dice roll handled separately.
    pub fn final_value(&self, dice_sum: i16) -> i16 {
        let mut val = dice_sum + match &self.modifiers.override_ {
            Some(v) => v.value.modifier as i16,
            None => self.base.value.modifier as i16,
        };
        for additive in &self.modifiers.additive {
            val += additive.value as i16;
        }
        val
    }
}
