use dioxus::prelude::*;
use strum::IntoEnumIterator;

use fage2e::{self, Advancement};

#[component]
pub fn Level1ClassSelections(
    mut class_selections: Signal<fage2e::Level1ClassSelections>,
    character: ReadOnlySignal<fage2e::Character>,
) -> Element {
    // Set up signals for the sub-advancement values.
    let envoy = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Envoy(v) => v.clone(),
            _ => fage2e::envoy::Level1Selections::default(),
        }
    });
    let mage = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Mage(v) => v.clone(),
            _ => fage2e::mage::Level1Selections::default(),
        }
    });
    let rogue = use_signal(move || {
        match &*class_selections.read() {
            fage2e::Level1ClassSelections::Rogue(v) => v.clone(),
            _ => fage2e::rogue::Level1Selections::default(),
        }
    });
    let warrior = use_signal(move || {
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

    // Set up an effect to produce character information for the class-specific selections
    // and to determine class selection completeness.
    let mut selection_status = use_signal(|| Result::<bool, ()>::Ok(false));
    let mut subcharacter = use_signal(fage2e::Character::new);
    use_effect(move || {
        let mut character = character();
        selection_status.set((*class_selections.read()).apply_self(&mut character));
        subcharacter.set(character);
    });

    use crate::widget::Selector;
    use crate::advancement::class::envoy::Level1Selections as EnvoyLevel1Selections;
    use crate::advancement::class::mage::Level1Selections as MageLevel1Selections;
    use crate::advancement::class::rogue::Level1Selections as RogueLevel1Selections;
    use crate::advancement::class::warrior::Level1Selections as WarriorLevel1Selections;
    use crate::styling::class_for_completeness;

    rsx! {
        div {
            class: class_for_completeness(selection_status()),
            h4 { class: "section-header", "Class Selection" }
            Selector { options, selection }
        }
        match selection() {
            None => None,
            Some(fage2e::Class::Envoy) => rsx! { EnvoyLevel1Selections { selections: envoy, character: subcharacter } },
            Some(fage2e::Class::Mage) => rsx! { MageLevel1Selections { selections: mage, character: subcharacter } },
            Some(fage2e::Class::Rogue) => rsx! { RogueLevel1Selections { selections: rogue, character: subcharacter } },
            Some(fage2e::Class::Warrior) => rsx! { WarriorLevel1Selections { selections: warrior, character: subcharacter } },
        }
    }
}
