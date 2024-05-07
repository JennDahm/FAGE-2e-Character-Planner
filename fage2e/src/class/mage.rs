#![allow(dead_code)]
//! Various details about the Mage class.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Ability, Advancement, Character, InitialWeaponGroups, WeaponGroup};

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Accuracy, Ability::Intelligence, Ability::Perception, Ability::Willpower,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Communication, Ability::Constitution, Ability::Dexterity, Ability::Fighting, Ability::Strength,
];

pub static STARTING_HEALTH: u8 = 20;

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

/// The initial weapon group selection for this class.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeaponGroupSelection {
    pub choices_: [Option<WeaponGroup>; 0],
}

impl InitialWeaponGroups for WeaponGroupSelection {
    fn always_get() -> &'static [WeaponGroup] {
        &[
            WeaponGroup::Brawling,
            WeaponGroup::Staves,
        ]
    }

    fn choose_between() -> &'static [WeaponGroup] {
        &[]
    }

    fn num_choices() -> usize {
        0
    }

    fn choices(&self) -> &[Option<WeaponGroup>] {
        &self.choices_
    }

    fn choices_mut(&mut self) -> &mut [Option<WeaponGroup>] {
        &mut self.choices_
    }
}

impl PartialEq for WeaponGroupSelection {
    fn eq(&self, other: &Self) -> bool {
        self.choices_ == other.choices_
    }
}
