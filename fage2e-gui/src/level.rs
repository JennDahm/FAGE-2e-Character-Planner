use std::ops::Deref;

use dioxus::prelude::*;
use tracing::info;

use fage2e;
use fage2e::Advancement;

#[component]
pub fn Level1(character: ReadOnlySignal<fage2e::Character>, mut level1: Signal<fage2e::Level1>) -> Element {
    // Set up signals for the sub-advancement values and effects to copy them back into
    // the level advancement.
    let name = use_signal(move || {
        (*level1.read()).name.clone()
    });
    use_effect(move || { (*level1.write()).name = name(); });

    // Set up signals for the sub-advancement states.
    let mut name_status = use_signal(|| Result::<bool, ()>::Ok(false));
    // Set up an effect to update sub-advancement states.
    use_effect(move || {
        let mut character = character();
        let level1 = level1.read();
        let level1 = level1.deref();

        let _ = level1.apply_self(&mut character);
        name_status.set(level1.name.apply_all(&mut character));

        info!("Updated level 1 info");
    });

    use crate::advancement::SelectName;

    rsx! {
        h3 { "Level 1" }
        SelectName { name }
    }
}
