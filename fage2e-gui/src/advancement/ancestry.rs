pub mod draak;
pub mod dwarf;
pub mod elf;
pub mod gnome;
pub mod goblin;
pub mod halfling;
pub mod human;
pub mod orc;
pub mod wildfolk;

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

    // Set up an effect to produce character information for the ancestry-specific selections
    // and to determine ancestry selection completeness.
    let mut selection_status = use_signal(|| Result::<bool, ()>::Ok(false));
    let mut subcharacter = use_signal(fage2e::Character::new);
    use_effect(move || {
        let mut character = character();
        selection_status.set((*ancestry_selections.read()).apply_self(&mut character));
        subcharacter.set(character);
    });

    use crate::widget::Selector;
    use crate::styling::class_for_completeness;
    use draak::Level1Selections as DraakLevel1Selections;
    use dwarf::Level1Selections as DwarfLevel1Selections;
    use elf::Level1Selections as ElfLevel1Selections;
    use gnome::Level1Selections as GnomeLevel1Selections;
    use goblin::Level1Selections as GoblinLevel1Selections;
    use halfling::Level1Selections as HalflingLevel1Selections;
    use human::Level1Selections as HumanLevel1Selections;
    use orc::Level1Selections as OrcLevel1Selections;
    use wildfolk::Level1Selections as WildfolkLevel1Selections;

    rsx! {
        div {
            class: class_for_completeness(selection_status()),
            h4 { class: "section-header", "Ancestry Selection" }
            Selector { options, selection }
        }
        match selection() {
            Some(fage2e::Ancestry::Draak) => rsx! { DraakLevel1Selections { selections: draak, character: subcharacter } },
            Some(fage2e::Ancestry::Dwarf) => rsx! { DwarfLevel1Selections { selections: dwarf, character: subcharacter } },
            Some(fage2e::Ancestry::Elf) => rsx! { ElfLevel1Selections { selections: elf, character: subcharacter } },
            Some(fage2e::Ancestry::Gnome) => rsx! { GnomeLevel1Selections { selections: gnome, character: subcharacter } },
            Some(fage2e::Ancestry::Goblin) => rsx! { GoblinLevel1Selections { selections: goblin, character: subcharacter } },
            Some(fage2e::Ancestry::Halfling) => rsx! { HalflingLevel1Selections { selections: halfling, character: subcharacter } },
            Some(fage2e::Ancestry::Human) => rsx! { HumanLevel1Selections { selections: human, character: subcharacter } },
            Some(fage2e::Ancestry::Orc) => rsx! { OrcLevel1Selections { selections: orc, character: subcharacter } },
            Some(fage2e::Ancestry::Wildfolk(_)) => rsx! { WildfolkLevel1Selections { selections: wildfolk, character: subcharacter } },
            None => None,
        }
    }
}
