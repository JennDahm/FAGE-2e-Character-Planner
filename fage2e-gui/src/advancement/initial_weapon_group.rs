use std::iter::repeat;

use dioxus::prelude::*;

use fage2e;

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
                    span { class: "pressable pressed", "{option}" }
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
