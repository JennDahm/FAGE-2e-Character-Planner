#![allow(dead_code)]
//! Advancements specifically related to character creation.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{Character, Dice, Ability, AbilityScore, Advancement, LeafNodeAdvancement, WeaponGroup};

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


/// A generic interface to an ancestry's benefit choices.
pub trait AncestryBenefit: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + Sized + IntoEnumIterator {
    /// The display name of this benefit.
    fn display_name(&self) -> String;

    /// For a given 2d6 roll, the corresponding benefit.
    fn from_roll(roll: u16) -> Result<Self, ()>;

    /// Whether this benefit counts as both choices when choosing manually.
    fn counts_as_two(&self) -> bool;

    /// Apply this benefit to the character.
    ///
    /// Returns whether the selection is complete.
    fn apply(&self, char: &mut Character) -> bool;
}

/// The user's selection of ancestry benefits.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AncestryBenefitSelections<B>
    where B : AncestryBenefit
{
    pub selection1: Option<B>,
    pub selection2: Option<B>,
    pub selections_were_rolled: bool,
}

// For whatever reason, if we try to derive Default, it requires that B also
// derive Default, which is generally undesirable.
impl <B> Default for AncestryBenefitSelections<B>
    where B : AncestryBenefit
{
    fn default() -> Self {
        Self {
            selection1: None,
            selection2: None,
            selections_were_rolled: false,
        }
    }
}

impl <B> AncestryBenefitSelections<B>
    where B : std::fmt::Debug + Clone + Copy + PartialEq + AncestryBenefit
{
    /// Randomly determine the Draak benefits to use.
    ///
    /// The user will still have to make sub-selections if Magical Resistance is
    /// rolled.
    #[cfg(feature = "rand")]
    pub fn roll() -> Self {
        let selection1 = B::from_roll(Dice::d6(2).roll_all_sum()).unwrap();
        let selection2 = loop {
            let selection = B::from_roll(Dice::d6(2).roll_all_sum()).unwrap();
            if selection != selection1 {
                break selection;
            }
        };
        Self {
            selection1: Some(selection1),
            selection2: Some(selection2),
            selections_were_rolled: true,
        }
    }
}

impl <B> LeafNodeAdvancement for AncestryBenefitSelections<B>
    where B : std::fmt::Debug + Clone + Copy + PartialEq + AncestryBenefit
{
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        // Apply the first selection, if it was selected.
        if let Some(selection1) = self.selection1 {
            let selection1_done = selection1.apply(char);

            // The user can't select the same benefit twice.
            if self.selection1 == self.selection2 {
                return Err(());
            }

            // If this counts as two selections and the user didn't roll for
            // their selections...
            if selection1.counts_as_two() && !self.selections_were_rolled {
                // ... then it's an error if the user made a second selection.
                if self.selection2.is_some() {
                    return Err(());
                }
                // Otherwise, the selections are done if the first selection
                // is done.
                else {
                    return Ok(selection1_done);
                }
            }
            // Otherwise...
            else {
                // ... apply the second selection, if it was selected.
                if let Some(selection2) = self.selection2 {
                    let selection2_done = selection2.apply(char);
                    return Ok(selection1_done && selection2_done);
                }
                // Otherwise, the user isn't done.
                else {
                    return Ok(false);
                }
            }
        }
        // If there was no first selection...
        else {
            // ... apply the second selection, if it was selected.
            if let Some(selection2) = self.selection2 {
                let selection2_done = selection2.apply(char);
                // The user is done IFF selection2 is done, selection2 counts as two
                // benefits, AND the user did not roll for their selections.
                return Ok(selection2_done && selection2.counts_as_two() && !self.selections_were_rolled);
            }
            // Otherwise, the user isn't done.
            else {
                return Ok(false);
            }
        }
    }
}
