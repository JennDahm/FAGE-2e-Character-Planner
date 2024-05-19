#![allow(dead_code)]
//! Generic leveling advancements that apply across all classes.

use std::cmp::max;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Ability, AdditiveModifier, Character, Dice, Focus, FocusLevel, ModifierSource};

/// A generic character advancement.
///
/// This may have sub-advancements. For example, at level 1 the player must choose a class,
/// and then based on that class they must make other choices. Choosing the class is an
/// advancement; the sub-choices are sub-advancements. Similarly, the entirety of a single
/// "level up the character" operation can be considered an advancement itself, with each
/// individual "normal" advancement being a sub-advancement of that level advancement.
pub trait Advancement {
    /// Apply just this advancement to the given character, but not its sub-advancements.
    ///
    /// This is useful in GUIs where you would want to present the user with all of their
    /// mistakes, not just the first one.
    ///
    /// Return Value:
    /// * Ok(true) if the advancement was fully filled out and applied successfully.
    /// * Ok(false) if the advancement wasn't fully filled out, but there weren't other problems.
    /// * Err() if there was an error and the advancement couldn't be applied.
    fn apply_self(&self, char: &mut Character) -> Result<bool, ()>;

    /// Apply this advancement and all its sub-advancements (recursively) to the given character.
    ///
    /// This is useful if you don't care about presenting mistakes in bulk; you only care
    /// about whether the entire thing is valid or not.
    ///
    /// Return Value:
    /// * Ok(true) if the advancement was fully filled out and applied successfully.
    /// * Ok(false) if the advancement wasn't fully filled out, but there weren't other problems.
    /// * Err() if there was an error and the advancement couldn't be applied.
    fn apply_all(&self, char: &mut Character) -> Result<bool, ()>
    {
        // Start with self, aborting early if there was an error.
        let self_done = self.apply_self(char)?;

        // Iterate over every subadvancement and fully apply each until the first error.
        let mut rollup: Result<bool, ()> = Ok(true);
        self.foreach(&mut |adv: &dyn Advancement| {
            rollup = rollup.and_then(|done| {
                // Be sure to avoid short-circuiting issues.
                Ok(adv.apply_all(char)? && done)
            })
        });

        rollup.map(|rollup_done| rollup_done && self_done)
    }

    /// Apply the given function to each sub-advancement (immutable access).
    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement));

    /// Apply the given function to each sub-advancement (mutable access).
    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement));

    /// Convert this to Any, so that we can downcast back to the original, concrete type.
    ///
    /// See this StackOverflow answer for why this is necessary:
    /// https://stackoverflow.com/a/33687996
    fn as_any(&self) -> &dyn std::any::Any;
}

/// A generic character advancement that explicitly doesn't have sub-advancements.
///
/// This helps avoid the boilerplate of specifying `foreach` and `foreach_mut` on
/// every little advancement.
pub trait LeafNodeAdvancement {
    /// Apply this advancement to the given character.
    ///
    /// Return Value:
    /// * Ok(true) if the advancement was fully filled out and applied successfully.
    /// * Ok(false) if the advancement wasn't fully filled out, but there weren't other problems.
    /// * Err() if there was an error and the advancement couldn't be applied.
    fn apply(&self, char: &mut Character) -> Result<bool, ()>;
}

/// A LeafNodeAdvancement is, itself, an Advancement.
impl<T: LeafNodeAdvancement + 'static> Advancement for T {
    fn apply_self(&self, char: &mut Character) -> Result<bool, ()> {
        self.apply(char)
    }

    fn foreach(&self, _: &mut dyn FnMut(&dyn Advancement)) {
    }
    fn foreach_mut(&mut self, _: &mut dyn FnMut(&mut dyn Advancement)) {
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// An incremental advance on the character's base defense.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DefenseAdvancement {
    // There's not a choice for the user to make here.
}

impl LeafNodeAdvancement for DefenseAdvancement {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        char.mechanical_properties.defense_advancements.push(
            AdditiveModifier {
                value: 1,
                source: ModifierSource::Level(char.mechanical_properties.level),
            }
        );
        Ok(true)
    }
}


/// An incremental advance on the character's health that includes a dice roll.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DiceBasedHealthAdvancement {
    /// The raw dice roll result, if it's been decided yet.
    /// This does not include constitution modifiers.
    pub roll_result: Option<u8>,
}

impl DiceBasedHealthAdvancement {
    /// The dice roll to use for this advancement.
    pub fn dice() -> Dice {
        Dice::d6(1)
    }

    /// The total advancement value, considering the minimum.
    ///
    /// NOTE: This does not check that the roll value itself was valid.
    ///
    /// Returns:
    /// * Ok(val) if no coercion happened.
    /// * Err(val) if the value was below the minimum advancement.
    pub fn calculated(&self, con: i8) -> Result<i8, i8> {
        let total = con + self.roll_result.unwrap_or(0) as i8;
        if total < 1 {
            Err(1)
        } else {
            Ok(total)
        }
    }
}

impl LeafNodeAdvancement for DiceBasedHealthAdvancement {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let roll = match self.roll_result {
            None => return Ok(false),
            Some(v) => v as i16,
        };
        let dice = Self::dice();
        if dice.min_value() <= roll && roll <= dice.max_value() {
            let total = self.calculated(char.mechanical_properties.abilities.get(Ability::Constitution).score);
            let total = match total {
                Ok(v) => v,
                Err(v) => v,
            };
            char.mechanical_properties.health_advancements.push(
                AdditiveModifier {
                    value: total,
                    source: ModifierSource::Level(char.mechanical_properties.level),
                }
            );
            Ok(true)
        }
        else {
            Err(())
        }
    }
}

/// An incremental advance on the character's health based solely on the character's Constitution.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstitutionBasedHealthAdvancement {
    // There's not a choice for the user to make here.
}

impl LeafNodeAdvancement for ConstitutionBasedHealthAdvancement {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let total = max(char.mechanical_properties.abilities.get(Ability::Constitution).score, 1);
        char.mechanical_properties.health_advancements.push(
            AdditiveModifier {
                value: total,
                source: ModifierSource::Level(char.mechanical_properties.level),
            }
        );
        Ok(true)
    }
}


/// An incremental advancement on a single primary ability score.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrimaryAbilityAdvancement {
    /// Which ability to advance, or None if not selected yet.
    pub ability: Option<Ability>,
}

impl LeafNodeAdvancement for PrimaryAbilityAdvancement {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let ability = match self.ability {
            None => return Ok(false),
            Some(a) => a,
        };

        let class = match char.mechanical_properties.class {
            None => return Err(()),
            Some(c) => c,
        };

        if class.primary_abilities().contains(&ability) {
            *char.mechanical_properties.abilities.get_mut(ability) += 1;
            Ok(true)
        }
        else {
            Err(())
        }
    }
}

/// An incremental advancement on a single secondary ability score.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecondaryAbilityAdvancement {
    /// Which ability to advance, or None if not selected yet.
    pub ability: Option<Ability>,
}

impl LeafNodeAdvancement for SecondaryAbilityAdvancement {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let ability = match self.ability {
            None => return Ok(false),
            Some(a) => a,
        };

        let class = match char.mechanical_properties.class {
            None => return Err(()),
            Some(c) => c,
        };

        if class.secondary_abilities().contains(&ability) {
            *char.mechanical_properties.abilities.get_mut(ability) += 1;
            Ok(true)
        }
        else {
            Err(())
        }
    }
}


fn advance_focus(char: &mut Character, focus: Focus) -> Result<(), ()> {
    if let Some(&current_level) = char.mechanical_properties.focuses.get(&focus) {
        // Can't triple focus.
        if current_level == FocusLevel::DoubleFocus {
            Err(())
        }
        // You can double focus, but only starting at level 11.
        else if char.mechanical_properties.level >= 11 {
            char.mechanical_properties.focuses.insert(focus, FocusLevel::DoubleFocus);
            Ok(())
        }
        // Otherwise you can't double focus yet.
        else {
            Err(())
        }
    }
    else {
        char.mechanical_properties.focuses.insert(focus, FocusLevel::SingleFocus);
        Ok(())
    }
}


/// An opportunity to pick a new focus or double-up on an already-taken focus in a primary ability.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrimaryFocusAdvancement {
    /// Which focus to take or advance.
    pub focus: Option<Focus>,
}

impl LeafNodeAdvancement for PrimaryFocusAdvancement {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let focus = match self.focus {
            None => return Ok(false),
            Some(f) => f,
        };

        let class = match char.mechanical_properties.class {
            None => return Err(()),
            Some(c) => c,
        };

        if !class.primary_abilities().contains(&focus.ability()) {
            return Err(());
        }
        match advance_focus(char, focus) {
            Ok(()) => Ok(true),
            Err(()) => Err(()),
        }
    }
}

/// An opportunity to pick a new focus or double-up on an already-taken focus in a secondary ability.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecondaryFocusAdvancement {
    /// Which focus to take or advance.
    pub focus: Option<Focus>,
}

impl LeafNodeAdvancement for SecondaryFocusAdvancement {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let focus = match self.focus {
            None => return Ok(false),
            Some(f) => f,
        };

        let class = match char.mechanical_properties.class {
            None => return Err(()),
            Some(c) => c,
        };

        if !class.secondary_abilities().contains(&focus.ability()) {
            return Err(());
        }
        match advance_focus(char, focus) {
            Ok(()) => Ok(true),
            Err(()) => Err(()),
        }
    }
}


/// An opportunity to pick a new stunt ability.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StuntAdvancement {
    // TODO: Option<Stunt>
}

/// An opportunity to pick a new out-of-class (or in-class) stunt ability.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OutOfClassStuntAdvancement {
    // TODO: Option<Stunt>
}


/// An opportunity to pick a new talent or advance an already-taken one.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TalentAdvancement {
    // TODO: Option<Talent>
}


/// An opportunity to pick a new specialization or advance an already-taken one.
///
/// Alternatively, you can take a regular talent, with some restrictions.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpecializationAdvancement {
    // TODO: Option<Specialization>
    // TODO: Option<Talent>
}
