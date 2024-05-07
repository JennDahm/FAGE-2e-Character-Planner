#![allow(dead_code)]
//! Various details about the Envoy class.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Ability, Advancement, Character, InitialWeaponGroups, WeaponGroup};

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Communication, Ability::Fighting, Ability::Intelligence, Ability::Willpower,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Accuracy, Ability::Constitution, Ability::Dexterity, Ability::Perception, Ability::Strength,
];

pub static STARTING_HEALTH: u8 = 25;

/// The initial selections the user must make for this class.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Level1Selections {
    pub weapon_groups: WeaponGroupSelection,
    // TODO: Level 1 powers
    // TODO: Starting Specialization
    // TODO: Starting Talents
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

pub const STARTING_WEAPON_GROUPS_NUM_CHOICES: usize = 3;

/// The initial weapon group selection for this class.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeaponGroupSelection {
    pub choices_: [Option<WeaponGroup>; STARTING_WEAPON_GROUPS_NUM_CHOICES],
}

impl InitialWeaponGroups for WeaponGroupSelection {
    fn always_get() -> &'static [WeaponGroup] {
        &[]
    }

    fn choose_between() -> &'static [WeaponGroup] {
        &[
            WeaponGroup::BlackPowder,
            WeaponGroup::Bludgeons,
            WeaponGroup::Bows,
            WeaponGroup::Brawling,
            WeaponGroup::HeavyBlades,
            WeaponGroup::LightBlades,
            WeaponGroup::Slings,
            WeaponGroup::Spears,
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

impl PartialEq for WeaponGroupSelection {
    fn eq(&self, other: &Self) -> bool {
        self.choices_ == other.choices_
    }
}

