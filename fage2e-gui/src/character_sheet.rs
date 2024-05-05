use dioxus::prelude::*;

use fage2e::*;
use strum::IntoEnumIterator;


#[component]
pub fn CharacterSheet(character: ReadOnlySignal<Character>) -> Element {
    let character = character();
    let bgrnd = character.flavor.background.unwrap_or("".to_owned());
    let social_class = character.flavor.social_class.unwrap_or("".to_owned());
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
                        td {
                            class: "ability-score",
                            rowspan: "2",
                            title: "Hovering!", // Look, ma, hover text!
                            "0"
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

