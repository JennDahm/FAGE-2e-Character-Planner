use std::iter::repeat;

use dioxus::prelude::*;
use strum::IntoEnumIterator;

use fage2e;
use tracing::info;

#[component]
pub fn Level1ClassSelections(mut class_selections: Signal<fage2e::Level1ClassSelections>) -> Element {
    // Set up signals for the sub-advancement values.
    let mut envoy = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Envoy(v) => v.clone(),
            _ => fage2e::envoy::Level1Selections::default(),
        }
    });
    let mut mage = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Mage(v) => v.clone(),
            _ => fage2e::mage::Level1Selections::default(),
        }
    });
    let mut rogue = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Rogue(v) => v.clone(),
            _ => fage2e::rogue::Level1Selections::default(),
        }
    });
    let mut warrior = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Warrior(v) => v.clone(),
            _ => fage2e::warrior::Level1Selections::default(),
        }
    });
    let selection = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Envoy(_) => Some(fage2e::Class::Envoy),
            fage2e::Level1ClassSelections::Mage(_) => Some(fage2e::Class::Mage),
            fage2e::Level1ClassSelections::Rogue(_) => Some(fage2e::Class::Rogue),
            fage2e::Level1ClassSelections::Warrior(_) => Some(fage2e::Class::Warrior),
            fage2e::Level1ClassSelections::NoChoice => None,
        }
    });
    let options = use_signal(|| fage2e::Class::iter().collect());

    // Set up an effect to copy the sub-advancements back into the top level advancement.
    use_effect(move || {
        class_selections.set(match selection() {
            Some(fage2e::Class::Envoy) => fage2e::Level1ClassSelections::Envoy(envoy()),
            Some(fage2e::Class::Mage) => fage2e::Level1ClassSelections::Mage(mage()),
            Some(fage2e::Class::Rogue) => fage2e::Level1ClassSelections::Rogue(rogue()),
            Some(fage2e::Class::Warrior) => fage2e::Level1ClassSelections::Warrior(warrior()),
            None => fage2e::Level1ClassSelections::NoChoice,
        });
    });

    // TEMPORARY: Weapon selection test
    let envoy_groups = use_signal(|| {
        (*envoy.read()).weapon_groups.clone()
    });
    use_effect(move || {
        (*envoy.write()).weapon_groups = envoy_groups();
    });
    let mage_groups = use_signal(|| {
        (*mage.read()).weapon_groups.clone()
    });
    use_effect(move || {
        (*mage.write()).weapon_groups = mage_groups();
    });
    let rogue_groups = use_signal(|| {
        (*rogue.read()).weapon_groups.clone()
    });
    use_effect(move || {
        (*rogue.write()).weapon_groups = rogue_groups();
    });
    let warrior_groups = use_signal(|| {
        (*warrior.read()).weapon_groups.clone()
    });
    use_effect(move || {
        (*warrior.write()).weapon_groups = warrior_groups();
    });

    use crate::widget::Selector;
    rsx! {
        h4 { class: "section", "Class Selection" }
        Selector { options, selection }
        // TODO: Sub-class selectors.
        match selection() {
            None => None,
            Some(fage2e::Class::Envoy) => rsx! { InitialWeaponGroups { groups: envoy_groups } },
            Some(fage2e::Class::Mage) => rsx! { InitialWeaponGroups { groups: mage_groups } },
            Some(fage2e::Class::Rogue) => rsx! { InitialWeaponGroups { groups: rogue_groups } },
            Some(fage2e::Class::Warrior) => rsx! { InitialWeaponGroups { groups: warrior_groups } },
        }
    }
}

#[component]
pub fn InitialWeaponGroups<T: fage2e::InitialWeaponGroups + PartialEq + Clone + 'static>(
    mut groups: Signal<T>
) -> Element {
    let options = use_signal(|| { T::choose_between().iter().map(|o| *o).collect() });
    let selections = use_signal(move || {
        let mut selections = Vec::new();
        for choice in (*groups.read()).choices() {
            if let Some(choice) = choice {
                selections.push(*choice)
            }
        }
        selections
    });
    let max_selections = use_signal(|| { T::num_choices() });

    use_effect(move || {
        info!("New weapon selection: {:?}", &*selections.read());

        let selections = &*selections.read();
        let groups = &mut *groups.write();

        // Transform the user's selections into an array of Option<WeaponGroup>,
        // extend it forever with None, then zip it to the InitialWeaponGroups
        // choices array. This will let us assign a valid value to each choice
        // slot.
        let selections = selections.iter().map(|s| Some(*s)).chain(repeat(None));
        for (selection, option_slot) in selections.zip(groups.choices_mut().iter_mut()) {
            *option_slot = selection;
        }
    });

    use crate::widget::MultiSelector;
    rsx! {
        h4 { class: "section", "Weapon Training" }
        if T::always_get().len() > 0 {
            p { class: "label", "Always get:" }
            div {
                class: "selector",
                for option in T::always_get() {
                    span { class: "selected", "{option}" }
                }
            }
        }
        if T::num_choices() > 0 {
            p { class: "label", "Choose {max_selections()}:" }
            MultiSelector { options, selections, max_selections }
        }
        else {
            p { class: "label", "No elective options." }
        }
    }
}
