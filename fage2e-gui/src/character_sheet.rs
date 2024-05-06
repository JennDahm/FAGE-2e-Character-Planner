use std::ops::Deref;

use dioxus::prelude::*;

use fage2e::*;
use strum::IntoEnumIterator;


#[component]
pub fn CharacterSheet(character: ReadOnlySignal<Character>) -> Element {
    let character = character.read();
    let character = character.deref();

    let bgrnd = character.flavor.background.clone().unwrap_or("".to_owned());
    let social_class = character.flavor.social_class.clone().unwrap_or("".to_owned());
    rsx! {
        div {
            class: "character-sheet",
            p {
                b {"Name:"} " {character.flavor.name}" br {}
                b {"Background:"} " {bgrnd}" br {}
                b {"Social Class:"} " {social_class}" br {}
                b {"Race:"} " TODO" br {}
            }

            table {
                class: "ability-table",
                for ability in Ability::iter() {
                    tr {
                        th {
                            class: "ability-name",
                            "{ability}"
                        }
                        th {
                            class: "ability-score",
                            rowspan: "2",
                            title: "Hovering!", // Look, ma, hover text!
                            "{character.mechanical_properties.abilities.get(ability)}"
                        }
                    }
                    tr {
                        td {
                            class: "ability-focuses",
                            "(todo)"
                        }
                    }
                }
            }
        }
    }
}

