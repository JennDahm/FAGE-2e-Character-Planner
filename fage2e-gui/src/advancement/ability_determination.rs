use std::ops::{Deref, DerefMut};

use dioxus::prelude::*;
use strum::IntoEnumIterator;

use fage2e;

#[component]
pub fn ManuallyEnterAbilities(mut abilities: Signal<fage2e::ManuallyEnterAbilities>) -> Element {
    rsx! {
        table {
            for ability in fage2e::Ability::iter() {
                tr {
                    td {
                        padding_right: "5px",
                        "{ability}: "
                    }
                    td {
                        input {
                            r#type: "number",
                            min: "-2",
                            max: "12",
                            onchange: move |event| {
                                let val: i32 = event.value().parse().unwrap_or(0);
                                let val = match val {
                                    ..=-2 => -2,
                                    -1..=11 => val as i8,
                                    12.. => 12,
                                };
                                match ability {
                                    fage2e::Ability::Accuracy => (*abilities.write()).accuracy = val,
                                    fage2e::Ability::Communication => (*abilities.write()).communication = val,
                                    fage2e::Ability::Constitution => (*abilities.write()).constitution = val,
                                    fage2e::Ability::Dexterity => (*abilities.write()).dexterity = val,
                                    fage2e::Ability::Fighting => (*abilities.write()).fighting = val,
                                    fage2e::Ability::Intelligence => (*abilities.write()).intelligence = val,
                                    fage2e::Ability::Perception => (*abilities.write()).perception = val,
                                    fage2e::Ability::Strength => (*abilities.write()).strength = val,
                                    fage2e::Ability::Willpower => (*abilities.write()).willpower = val,
                                }
                            },
                            value: match ability {
                                fage2e::Ability::Accuracy => (*abilities.read()).accuracy,
                                fage2e::Ability::Communication => (*abilities.read()).communication,
                                fage2e::Ability::Constitution => (*abilities.read()).constitution,
                                fage2e::Ability::Dexterity => (*abilities.read()).dexterity,
                                fage2e::Ability::Fighting => (*abilities.read()).fighting,
                                fage2e::Ability::Intelligence => (*abilities.read()).intelligence,
                                fage2e::Ability::Perception => (*abilities.read()).perception,
                                fage2e::Ability::Strength => (*abilities.read()).strength,
                                fage2e::Ability::Willpower => (*abilities.read()).willpower,
                            }.to_string(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AbilityDetermination(mut abilities: Signal<fage2e::AbilityDetermination>) -> Element {
    let manual_abilities = use_signal(move || {
        match abilities.read().deref() {
            fage2e::AbilityDetermination::Manual(abilities) => abilities.clone(),
            _ => fage2e::ManuallyEnterAbilities::default(),
        }
    });
    use_effect(move || {
        match abilities.write().deref_mut() {
            fage2e::AbilityDetermination::Manual(abilities) => *abilities = manual_abilities(),
            // TODO: Other kinds of ability determination.
            _ => (),
        }
    });

    rsx! {
        h4 { class: "section", "Initial Abilities" }
        match *abilities.read() {
            fage2e::AbilityDetermination::Manual(_) => rsx! {
                ManuallyEnterAbilities { abilities: manual_abilities }
            },
            _ => rsx! { "<unsupported>" }
        }
    }
}
