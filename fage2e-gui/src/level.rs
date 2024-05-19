use std::ops::Deref;

use dioxus::prelude::*;
use tracing::info;

use fage2e;
use fage2e::Advancement;

#[component]
pub fn Level1(character: ReadOnlySignal<fage2e::Character>, mut level1: Signal<fage2e::Level1>) -> Element {
    // Set up signals for the sub-advancement values and effects to copy them back into
    // the level advancement.
    let name = use_signal(move || { (*level1.read()).name.clone() });
    use_effect(move || { (*level1.write()).name = name(); });

    let abilities = use_signal(move || {
        // TODO: Allow the ability to select ability determination style.
        let level1 = level1.read();
        let level1 = level1.deref();
        match level1.abilities {
            fage2e::AbilityDetermination::Manual(_) => level1.abilities.clone(),
            _ => fage2e::AbilityDetermination::Manual(fage2e::ManuallyEnterAbilities::default()),
        }
    });
    use_effect(move || { (*level1.write()).abilities = abilities(); });

    let class_selections = use_signal(move || { (*level1.read()).class.clone() });
    use_effect(move || { (*level1.write()).class = class_selections(); });

    let ancestry_selections = use_signal(move || { (*level1.read()).ancestry.clone() });
    use_effect(move || { (*level1.write()).ancestry = ancestry_selections(); });

    let health = use_signal(move || { (*level1.read()).health.clone() });
    use_effect(move || { (*level1.write()).health = health(); });

    // Set up signals for the sub-advancement states.
    let mut name_status = use_signal(|| Result::<bool, ()>::Ok(false));
    let mut abilities_status = use_signal(|| Result::<bool, ()>::Ok(false));
    let mut class_selections_character = use_signal(fage2e::Character::new);
    let mut health_status = use_signal(|| Result::<bool, ()>::Ok(false));

    // Set up a signal for communicating constitution.
    let mut constitution = use_signal(|| 0);

    // Set up an effect to update sub-advancement states and calculate constitution.
    use_effect(move || {
        let mut character = character();
        let level1 = level1.read();
        let level1 = level1.deref();

        let _ = level1.apply_self(&mut character);
        name_status.set(level1.name.apply_all(&mut character));
        abilities_status.set(level1.abilities.apply_all(&mut character));

        class_selections_character.set(character.clone());
        let _ = level1.class.apply_all(&mut character);

        health_status.set(level1.health.apply_all(&mut character));

        constitution.set(character.mechanical_properties.abilities.get(fage2e::Ability::Constitution).score);

        info!("Updated level 1 info");
    });

    use crate::advancement::SelectName;
    use crate::advancement::AbilityDetermination;
    use crate::advancement::Level1ClassSelections;
    use crate::advancement::Level1AncestrySelections;
    use crate::advancement::DiceBasedHealthAdvancement;
    use crate::styling::class_for_completeness;

    rsx! {
        h3 { class: "title", "Level 1" }
        div {
            class: class_for_completeness(name_status()),
            SelectName { name }
        }
        div {
            class: class_for_completeness(abilities_status()),
            AbilityDetermination { abilities }
        }
        hr {}
        Level1ClassSelections { class_selections, character: class_selections_character }
        hr {}
        div {
            class: class_for_completeness(health_status()),
            DiceBasedHealthAdvancement { advancement: health, constitution }
        }
        hr {}
        // TODO: Make the right character
        Level1AncestrySelections { ancestry_selections, character: class_selections_character}
    }
}
