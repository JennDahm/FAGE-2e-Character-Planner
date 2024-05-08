use dioxus::prelude::*;

use fage2e;

#[component]
pub fn Level1Selections(mut selections: Signal<fage2e::mage::Level1Selections>) -> Element {
    let weapon_groups = use_signal(|| (*selections.read()).weapon_groups.clone());
    use_effect(move || { (*selections.write()).weapon_groups = weapon_groups(); });

    use crate::advancement::InitialWeaponGroups;

    rsx! {
        InitialWeaponGroups { groups: weapon_groups }
    }
}
