#![allow(dead_code)]
//! Advancements specifically related to character creation.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Character, Ability, AbilityScore, Advancement, LeafNodeAdvancement, WeaponGroup};

/// Character name selection
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectName
{
    pub name: String,
}

impl LeafNodeAdvancement for SelectName {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        char.flavor.name = self.name.clone();
        Ok(!self.name.is_empty())
    }
}

/// The player has chosen to select their abilities manually.
///
/// They will have 13 advancements they can make, but cannot advance anything past 3.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectAbilities {
    pub advancements: [Option<Ability>; 13],
}

impl LeafNodeAdvancement for SelectAbilities {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let abilities = &mut char.mechanical_properties.abilities;
        let mut all_done = true;
        for adv in self.advancements {
            if let Some(ability) = adv {
                if abilities.get(ability).score >= 3 {
                    return Err(());
                }
                abilities.get_mut(ability).score += 1;
            }
            else {
                all_done = false;
            }
        }
        Ok(all_done)
    }
}

/// The player has chosen to manually enter their abilities that they determined outside this program.
///
/// These will not be validated.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ManuallyEnterAbilities {
    pub accuracy: i8,
    pub communication: i8,
    pub constitution: i8,
    pub dexterity: i8,
    pub fighting: i8,
    pub intelligence: i8,
    pub perception: i8,
    pub strength: i8,
    pub willpower: i8,
}

impl LeafNodeAdvancement for ManuallyEnterAbilities {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        let abilities = &mut char.mechanical_properties.abilities;
        abilities.set(Ability::Accuracy, AbilityScore { score: self.accuracy, partial: 0});
        abilities.set(Ability::Communication, AbilityScore { score: self.communication, partial: 0});
        abilities.set(Ability::Constitution, AbilityScore { score: self.constitution, partial: 0});
        abilities.set(Ability::Dexterity, AbilityScore { score: self.dexterity, partial: 0});
        abilities.set(Ability::Fighting, AbilityScore { score: self.fighting, partial: 0});
        abilities.set(Ability::Intelligence, AbilityScore { score: self.intelligence, partial: 0});
        abilities.set(Ability::Perception, AbilityScore { score: self.perception, partial: 0});
        abilities.set(Ability::Strength, AbilityScore { score: self.strength, partial: 0});
        abilities.set(Ability::Willpower, AbilityScore { score: self.willpower, partial: 0});
        Ok(true)
    }
}

/// The player has different choices for how to determine their starting abilities.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AbilityDetermination {
    #[default]
    NoChoice,
    Select(SelectAbilities),
    // TODO: Dice-roll based method.
    Manual(ManuallyEnterAbilities),
}

/// This is just a wrapper for the individual sub-advancements. Making it an Advancement itself
/// has advantages in the GUI code, though, because the user does need to decide how to decide
/// abilities.
impl Advancement for AbilityDetermination {
    fn apply_self(&self, _: &mut Character) -> Result<bool, ()> {
        // Nothing to do here.
        Ok(true)
    }

    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement)) {
        match self {
            Self::NoChoice => (),
            Self::Select(s) => f(s),
            Self::Manual(s) => f(s),
        }
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        match self {
            Self::NoChoice => (),
            Self::Select(s) => f(s),
            Self::Manual(s) => f(s),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}


/// A generic interface to the player's initial weapon group options.
///
/// This is class-dependent.
pub trait InitialWeaponGroups {
    /// A static list of the weapon groups this class always gets training in.
    fn always_get() -> &'static [WeaponGroup];

    /// A static list of the weapon groups this class can choose to be trained in.
    fn choose_between() -> &'static [WeaponGroup];

    /// The number of items in `choose_between()` the player can select.
    fn num_choices() -> usize;

    /// The player's current choices.
    fn choices(&self) -> &[Option<WeaponGroup>];

    /// Mutable reference to the player's current choices.
    fn choices_mut(&mut self) -> &mut [Option<WeaponGroup>];
}

impl<T: InitialWeaponGroups> LeafNodeAdvancement for T {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        // Apply the always-get training.
        for weapon_group in Self::always_get() {
            char.mechanical_properties.weapon_training.insert(*weapon_group);
        }

        // Apply the player's choices, keeping track of whether there were any
        // unselected choices or errors.
        let mut any_unselected = false;
        let mut any_err = false;
        for maybe_weapon_group in self.choices() {
            let weapon_group = match maybe_weapon_group {
                None => { any_unselected = true; continue; },
                Some(w) => w,
            };

            // It's an error if the user selects something outside the valid selection set.
            if !Self::choose_between().contains(&weapon_group) {
                any_err = true;
                continue;
            }

            char.mechanical_properties.weapon_training.insert(*weapon_group);
        }

        return if any_err { Err(()) } else { Ok(!any_unselected) };
    }
}
