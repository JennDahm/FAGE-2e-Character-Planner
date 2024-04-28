#![allow(dead_code)]
//! Various details about the Envoy class.

use crate::{Ability, Advancement, Character, LeafNodeAdvancement, WeaponGroup};

pub static PRIMARY_ABILITIES: [Ability; 4] = [
    Ability::Communication, Ability::Fighting, Ability::Intelligence, Ability::Willpower,
];
pub static SECONDARY_ABILITIES: [Ability; 5] = [
    Ability::Accuracy, Ability::Constitution, Ability::Dexterity, Ability::Perception, Ability::Strength,
];

pub static STARTING_HEALTH: u8 = 25;

/// The initial selections the user must make for this class.
pub struct Level1Selections {
    weapon_groups: WeaponGroupSelection,
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

pub static STARTING_WEAPON_GROUPS_CHOICE_BETWEEN: &'static [WeaponGroup] = &[
    WeaponGroup::BlackPowder,
    WeaponGroup::Bludgeons,
    WeaponGroup::Bows,
    WeaponGroup::Brawling,
    WeaponGroup::HeavyBlades,
    WeaponGroup::LightBlades,
    WeaponGroup::Slings,
    WeaponGroup::Spears,
];
pub const STARTING_WEAPON_GROUPS_NUM_CHOICES: usize = 3;

/// The initial weapon group selection for this class.
pub struct WeaponGroupSelection {
    choices: [Option<WeaponGroup>; STARTING_WEAPON_GROUPS_NUM_CHOICES],
}

impl LeafNodeAdvancement for WeaponGroupSelection {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        // This class doesn't have any common weapon group training.

        // Register the user's additional choices.
        crate::character_creation::apply_initial_weapon_group_selection(
            char,
            &STARTING_WEAPON_GROUPS_CHOICE_BETWEEN,
            &self.choices,
        )
    }
}
