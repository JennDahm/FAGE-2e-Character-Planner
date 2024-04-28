#![allow(dead_code)]
//! Various details about the Mage class.

use crate::{Ability, Advancement, Character, LeafNodeAdvancement, WeaponGroup};

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
pub struct WeaponGroupSelection {
}

impl LeafNodeAdvancement for WeaponGroupSelection {
    fn apply(&self, char: &mut Character) -> Result<bool, ()> {
        // Add common weapon group training.
        for weapon_group in [WeaponGroup::Brawling, WeaponGroup::Staves] {
            char.mechanical_properties.weapon_training.insert(weapon_group);
        }

        // This class doesn't have any other weapon group training choices.
        Ok(true)
    }
}
