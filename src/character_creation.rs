#![allow(dead_code)]
//! Advancements specifically related to character creation.

use crate::{character::Character, Ability, AbilityScore, Advancement, LeafNodeAdvancement, WeaponGroup};

/// The player has chosen to select their abilities manually.
///
/// They will have 13 advancements they can make, but cannot advance anything past 3.
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
pub struct ManuallyEnterAbilities {
    accuracy: i8,
    communication: i8,
    constitution: i8,
    dexterity: i8,
    fighting: i8,
    intelligence: i8,
    perception: i8,
    strength: i8,
    willpower: i8,
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
pub enum AbilityDetermination {
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

/// Common logic for applying the user's initial weapon group training selection.
///
/// The parameters for this are per-class.
///
/// Arguments:
/// * char: The character to apply the user's selection to.
/// * always_apply: The weapon group training to always apply.
/// * choose_between: The valid weapon group selections.
/// * choices: The user's choices.
///
/// Return:
/// * Ok(true) if all selections were made without errors.
/// * Ok(false) if any selections were not made.
/// * Err() if the user made invalid selections.
pub fn apply_initial_weapon_group_selection(
    char: &mut Character,
    always_apply: &[WeaponGroup],
    choose_between: &[WeaponGroup],
    choices: &[Option<WeaponGroup>],
) -> Result<bool, ()> {
    // Start by applying the "always apply" groups.
    let training = &mut char.mechanical_properties.weapon_training;
    for weapon_group in always_apply {
        // It's not an error if the character is already trained in this.
        if training.contains(&weapon_group) {
            continue;
        }
        training.push(*weapon_group);
    }

    // Now apply the player's choices, keeping track of whether there were any unselected
    // choices.
    let mut any_unselected = false;
    for maybe_weapon_group in choices {
        let weapon_group = match maybe_weapon_group {
            None => {any_unselected = true; continue; },
            Some(w) => w,
        };
        // It *is* an error if the user selects something outside the valid selection set.
        if !choose_between.contains(&weapon_group) {
            return Err(());
        }
        // It's weird, but not an error if the character is already trained in this.
        if training.contains(&weapon_group) {
            continue;
        }
        training.push(*weapon_group);
    }

    return Ok(!any_unselected);
}
