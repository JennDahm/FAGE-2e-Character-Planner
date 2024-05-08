#![allow(dead_code)]
//! Various details about the Rogue class.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Ability, Advancement, Character, InitialWeaponGroups, WeaponGroup};

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Accuracy, Ability::Communication, Ability::Dexterity, Ability::Perception,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Constitution, Ability::Fighting, Ability::Intelligence, Ability::Strength, Ability::Willpower,
];

pub static STARTING_HEALTH: u8 = 25;

/// The initial selections the user must make for this class.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Level1Selections {
    pub weapon_groups: WeaponGroupSelection,
}

/// This top-level advancement doesn't itself have any logic, but it has sub-advancements.
impl Advancement for Level1Selections {
    fn apply_self(&self, _: &mut Character) -> Result<bool, ()> {
        Ok(true)
    }

    fn foreach(&self, f: &mut dyn FnMut(&dyn Advancement)) {
        f(&self.weapon_groups);
    }

    fn foreach_mut(&mut self, f: &mut dyn FnMut(&mut dyn Advancement)) {
        f(&mut self.weapon_groups);
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}

pub const STARTING_WEAPON_GROUPS_NUM_CHOICES: usize = 2;

/// The initial weapon group selection for this class.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeaponGroupSelection {
    pub choices_: [Option<WeaponGroup>; STARTING_WEAPON_GROUPS_NUM_CHOICES],
}

impl InitialWeaponGroups for WeaponGroupSelection {
    fn always_get() -> &'static [WeaponGroup] {
        &[
            WeaponGroup::LightBlades,
            WeaponGroup::Staves,
        ]
    }

    fn choose_between() -> &'static [WeaponGroup] {
        &[
            WeaponGroup::BlackPowder,
            WeaponGroup::Bows,
            WeaponGroup::Brawling,
            WeaponGroup::Slings,
            WeaponGroup::Dueling,
        ]
    }

    fn num_choices() -> usize {
        STARTING_WEAPON_GROUPS_NUM_CHOICES
    }

    fn choices(&self) -> &[Option<WeaponGroup>] {
        &self.choices_
    }

    fn choices_mut(&mut self) -> &mut [Option<WeaponGroup>] {
        &mut self.choices_
    }
}
