use dioxus::prelude::*;
use strum::IntoEnumIterator;

use fage2e::{self, Advancement};

#[component]
pub fn Level1AncestrySelections(
    mut ancestry_selections: Signal<fage2e::Level1AncestrySelections>,
    character: ReadOnlySignal<fage2e::Character>,
) -> Element {
    // Set up signals for the sub-advancement values.
    let draak = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Draak(v) => v.clone(),
            _ => fage2e::draak::Level1Selections::default(),
        }
    });
    let dwarf = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Dwarf(v) => v.clone(),
            _ => fage2e::dwarf::Level1Selections::default(),
        }
    });
    let elf = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Elf(v) => v.clone(),
            _ => fage2e::elf::Level1Selections::default(),
        }
    });
    let gnome = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Gnome(v) => v.clone(),
            _ => fage2e::gnome::Level1Selections::default(),
        }
    });
    let goblin = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Goblin(v) => v.clone(),
            _ => fage2e::goblin::Level1Selections::default(),
        }
    });
    let halfling = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Halfling(v) => v.clone(),
            _ => fage2e::halfling::Level1Selections::default(),
        }
    });
    let human = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Human(v) => v.clone(),
            _ => fage2e::human::Level1Selections::default(),
        }
    });
    let orc = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Orc(v) => v.clone(),
            _ => fage2e::orc::Level1Selections::default(),
        }
    });
    let wildfolk = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::Wildfolk(v) => v.clone(),
            _ => fage2e::wildfolk::Level1Selections::default(),
        }
    });

    let selection = use_signal(move || {
        match &*ancestry_selections.read() {
            fage2e::Level1AncestrySelections::NoChoice => None,
            fage2e::Level1AncestrySelections::Draak(_) => Some(fage2e::Ancestry::Draak),
            fage2e::Level1AncestrySelections::Dwarf(_) => Some(fage2e::Ancestry::Dwarf),
            fage2e::Level1AncestrySelections::Elf(_) => Some(fage2e::Ancestry::Elf),
            fage2e::Level1AncestrySelections::Gnome(_) => Some(fage2e::Ancestry::Gnome),
            fage2e::Level1AncestrySelections::Goblin(_) => Some(fage2e::Ancestry::Goblin),
            fage2e::Level1AncestrySelections::Halfling(_) => Some(fage2e::Ancestry::Halfling),
            fage2e::Level1AncestrySelections::Human(_) => Some(fage2e::Ancestry::Human),
            fage2e::Level1AncestrySelections::Orc(_) => Some(fage2e::Ancestry::Orc),
            fage2e::Level1AncestrySelections::Wildfolk(_) => Some(fage2e::Ancestry::Wildfolk(None)),
        }
    });
    let options = use_signal(|| fage2e::Ancestry::iter().collect());

    // Set up an effect to copy the sub-advancements back into the top level advancement.
    use_effect(move || {
        ancestry_selections.set(match selection() {
            None => fage2e::Level1AncestrySelections::NoChoice,
            Some(fage2e::Ancestry::Draak) => fage2e::Level1AncestrySelections::Draak(draak()),
            Some(fage2e::Ancestry::Dwarf) => fage2e::Level1AncestrySelections::Dwarf(dwarf()),
            Some(fage2e::Ancestry::Elf) => fage2e::Level1AncestrySelections::Elf(elf()),
            Some(fage2e::Ancestry::Gnome) => fage2e::Level1AncestrySelections::Gnome(gnome()),
            Some(fage2e::Ancestry::Goblin) => fage2e::Level1AncestrySelections::Goblin(goblin()),
            Some(fage2e::Ancestry::Halfling) => fage2e::Level1AncestrySelections::Halfling(halfling()),
            Some(fage2e::Ancestry::Human) => fage2e::Level1AncestrySelections::Human(human()),
            Some(fage2e::Ancestry::Orc) => fage2e::Level1AncestrySelections::Orc(orc()),
            Some(fage2e::Ancestry::Wildfolk(_)) => fage2e::Level1AncestrySelections::Wildfolk(wildfolk()),
        });
    });

    use crate::widget::Selector;

    rsx! {
        div {
            h4 { class: "section-header", "Ancestry Selection" }
            Selector { options, selection }
        }
    }
}
