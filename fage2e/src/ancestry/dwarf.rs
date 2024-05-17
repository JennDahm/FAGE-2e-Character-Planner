#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Advancement, Character};

/// The initial selections the user must make for this ancestry.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Level1Selections {
    // TODO
}

/// This top-level advancement doesn't itself have any logic, but it has sub-advancements.
impl Advancement for Level1Selections {
    fn apply_self(&self, _: &mut Character) -> Result<bool, ()> {
        Ok(true)
    }

    fn foreach(&self, _: &mut dyn FnMut(&dyn Advancement)) {
        // TODO
    }

    fn foreach_mut(&mut self, _: &mut dyn FnMut(&mut dyn Advancement)) {
        // TODO
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}
