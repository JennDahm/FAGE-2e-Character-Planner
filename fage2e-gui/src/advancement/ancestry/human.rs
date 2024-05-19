use std::ops::Deref;

use dioxus::prelude::*;

use fage2e::{self, Advancement};
use strum::IntoEnumIterator;

#[component]
pub fn Level1Selections(
    mut selections: Signal<fage2e::human::Level1Selections>,
    character: ReadOnlySignal<fage2e::Character>,
) -> Element {
    // Set up signals for the sub-advancement values.
    let ability_focus = use_signal(|| (*selections.read()).ability_focus);
    use_effect(move || { (*selections.write()).ability_focus = ability_focus(); });
    let ability_focus_options = use_signal(|| fage2e::human::AbilityFocusSelection::iter().collect());

    // Set up signals for the sub-advancement states.
    let mut ability_focus_status = use_signal(|| Result::<bool, ()>::Ok(false));

    // Set up an effect to update sub-advancement states.
    use_effect(move || {
        let mut character = character();
        let selections = selections.read();
        let selections = selections.deref();

        ability_focus_status.set(selections.ability_focus.apply_all(&mut character));
    });

    use crate::styling::class_for_completeness;
    use crate::widget::Selector;

    rsx! {
        div {
            class: class_for_completeness(ability_focus_status()),
            h4 { class: "section-header", "Select Ability Focus" }
            Selector { options: ability_focus_options, selection: ability_focus }
        }
    }
}
