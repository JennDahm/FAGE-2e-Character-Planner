use dioxus::prelude::*;
use strum::IntoEnumIterator;

use fage2e;

#[component]
pub fn Level1ClassSelections(mut class_selections: Signal<fage2e::Level1ClassSelections>) -> Element {
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

    use crate::widget::Selector;
    use crate::advancement::envoy::Level1Selections as EnvoyLevel1Selections;
    use crate::advancement::mage::Level1Selections as MageLevel1Selections;
    use crate::advancement::rogue::Level1Selections as RogueLevel1Selections;
    use crate::advancement::warrior::Level1Selections as WarriorLevel1Selections;

    rsx! {
        h4 { class: "section", "Class Selection" }
        Selector { options, selection }
        match selection() {
            None => None,
            Some(fage2e::Class::Envoy) => rsx! { EnvoyLevel1Selections { selections: envoy } },
            Some(fage2e::Class::Mage) => rsx! { MageLevel1Selections { selections: mage } },
            Some(fage2e::Class::Rogue) => rsx! { RogueLevel1Selections { selections: rogue } },
            Some(fage2e::Class::Warrior) => rsx! { WarriorLevel1Selections { selections: warrior } },
        }
    }
}
