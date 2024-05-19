use std::ops::Deref;

use dioxus::prelude::*;

use fage2e::{self, Advancement};

#[component]
pub fn Level1Selections(
    mut selections: Signal<fage2e::envoy::Level1Selections>,
    character: ReadOnlySignal<fage2e::Character>,
) -> Element {
    // Set up signals for the sub-advancement values.
    let weapon_groups = use_signal(|| (*selections.read()).weapon_groups.clone());
    use_effect(move || { (*selections.write()).weapon_groups = weapon_groups(); });

    // Set up signals for the sub-advancement states.
    let mut weapon_groups_status = use_signal(|| Result::<bool, ()>::Ok(false));

    // Set up an effect to update sub-advancement states.
    use_effect(move || {
        let mut character = character();
        let selections = selections.read();
        let selections = selections.deref();

        weapon_groups_status.set(selections.weapon_groups.apply_all(&mut character));
    });

    use crate::advancement::InitialWeaponGroups;
    use crate::styling::class_for_completeness;

    rsx! {
        div {
            class: class_for_completeness(weapon_groups_status()),
            InitialWeaponGroups { groups: weapon_groups }
        }
    }
}
