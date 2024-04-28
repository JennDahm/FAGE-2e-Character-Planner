#![allow(dead_code)]
//! Various details about the Mage class.

use crate::{Ability, Advancement, LeafNodeAdvancement, WeaponGroup};

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Accuracy, Ability::Intelligence, Ability::Perception, Ability::Willpower,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Communication, Ability::Constitution, Ability::Dexterity, Ability::Fighting, Ability::Strength,
];

pub static STARTING_HEALTH: u8 = 20;

/// The initial selections the user must make for this class.
pub struct Level1Selections {
    weapon_groups: WeaponGroupSelection,
}

/// This top-level advancement doesn't itself have any logic, but it has sub-advancements.
impl Advancement for Level1Selections {
    fn apply_self(&self, _: &mut crate::character::Character) -> Result<bool, ()> {
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

pub static STARTING_WEAPON_GROUPS_ALWAYS: &'static [WeaponGroup] = &[
    WeaponGroup::Brawling, WeaponGroup::Staves,
];
pub static STARTING_WEAPON_GROUPS_CHOICE_BETWEEN: &'static [WeaponGroup] = &[];
pub const STARTING_WEAPON_GROUPS_NUM_CHOICES: usize = 0;

/// The initial weapon group selection for this class.
pub struct WeaponGroupSelection {
    choices: [Option<WeaponGroup>; STARTING_WEAPON_GROUPS_NUM_CHOICES],
}

impl LeafNodeAdvancement for WeaponGroupSelection {
    fn apply(&self, char: &mut crate::character::Character) -> Result<bool, ()> {
        crate::character_creation::apply_initial_weapon_group_selection(
            char,
            &STARTING_WEAPON_GROUPS_ALWAYS,
            &STARTING_WEAPON_GROUPS_CHOICE_BETWEEN,
            &self.choices,
        )
    }
}
