use std::{collections::HashMap, ops::Deref};

use dioxus::prelude::*;

use fage2e::*;
use strum::IntoEnumIterator;


#[component]
pub fn CharacterSheet(character: ReadOnlySignal<Character>) -> Element {
    let character_ref = character.read();
    let character_ref = character_ref.deref();

    let bgrnd = character_ref.flavor.background.clone().unwrap_or("".to_owned());
    let social_class = character_ref.flavor.social_class.clone().unwrap_or("".to_owned());
    let class = character_ref.mechanical_properties.class.clone().map(|c| c.to_string()).unwrap_or("".to_owned());
    rsx! {
        div {
            class: "character-sheet",
            div {
                display: "flex",
                flex_direction: "row",
                gap: "10px",
                margin_bottom: "10px",
                div {
                    p {
                        b {"Name:"} " {character_ref.flavor.name}" br {}
                        b {"Background:"} " {bgrnd}" br {}
                        b {"Social Class:"} " {social_class}" br {}
                    }
                    AbilityTable { character }
                }
                div {
                    p {
                        b {"Level:"} " {character_ref.mechanical_properties.level}" br {}
                        b {"Class:"} " {class}" br {}
                        b {"Race:"} " TODO" br {}
                    }
                    CoreStats { character }
                    Powers { character }
                    MeleeWeapons { character }
                }
            }
            RangeWeapons { character }
        }
    }
}


#[component]
fn AbilityTable(character: ReadOnlySignal<Character>) -> Element {
    let character = character.read();
    let character = character.deref();

    // Assemble a map of ability -> focus list
    let mut focuses = HashMap::new();
    for ability in fage2e::Ability::iter() {
        focuses.insert(ability, Vec::new());
    }
    for (focus, level) in character.mechanical_properties.focuses.iter() {
        let ability = focus.ability();
        let base_name = focus.base_name();
        let list = focuses.get_mut(&ability).unwrap();
        match *level {
            fage2e::FocusLevel::None => continue,
            fage2e::FocusLevel::SingleFocus => list.push(format!("{base_name}")),
            fage2e::FocusLevel::DoubleFocus => list.push(format!("{base_name}^2")),
        }
    }

    // TODO: Temporary.
    {
        let list = focuses.get_mut(&fage2e::Ability::Accuracy).unwrap();
        list.push(fage2e::Focus::AccuracyBrawling.base_name().to_owned());
        list.push(fage2e::Focus::AccuracyLightBlades.base_name().to_owned());
        list.push(fage2e::Focus::AccuracyDueling.base_name().to_owned());
    }

    let focuses = focuses.iter().map(|(k, v)| (k, v.join(", "))).collect::<HashMap<_, _>>();

    rsx! {
        table {
            class: "ability-table",
            for ability in Ability::iter() {
                tr {
                    th {
                        class: "ability-name hover-container",
                        "{ability}"
                        // Different hover-text style that lets us actually style things.
                        div {
                            class: "hover-content",
                            i { "Hello " } " " b { "hover" }
                        }
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
                        "{focuses.get(&ability).unwrap()}"
                    }
                }
            }
        }
    }
}


#[component]
fn CoreStats(character: ReadOnlySignal<Character>) -> Element {
    let character = character.read();
    let character = character.deref();

    let speed_details = character.speed_value();

    let defense_details = character.defense_value();

    rsx! {
        table {
            class: "stat-table",
            tr {
                class: "heading",
                th {
                    title: "How many yards you can move easily.",
                    "SPEED"
                }
                th {
                    title: "What an enemy has to beat to hit you.",
                    "DEFENSE"
                }
                th {
                    title: "Standard damage reduction.",
                    "ARMOR"
                }
                th { "HEALTH" }
            }
            tr {
                th {
                    title: "{format_value_modifiers(&speed_details)}",
                    "{speed_details.final_value()}"
                }
                th {
                    title: "{format_value_modifiers(&defense_details)}",
                    "{defense_details.final_value()}"
                }
                th {
                    "todo"
                }
                th {
                    "{character.status.health}/{character.mechanical_properties.max_health}"
                }
            }
        }
        table {
            class: "stat-table",
            tr {
                class: "heading",
                th {
                    title: "How far you can move in yards for a minor action.",
                    "MOVE"
                }
                th {
                    title: "How far you can move in yards for a 'charge' major action.",
                    "CHARGE"
                }
                th {
                    title: "How far you can move in yards for a 'run' major action.",
                    "RUN"
                }
            }
            tr {
                th {
                    title: "Your speed in yards; no less than 0.",
                    "{character.move_speed_yards()}"
                }
                th {
                    title: "Move speed divided by 2, rounded up.",
                    "{character.charge_speed_yards()}"
                }
                th {
                    title: "Move speed times 2.",
                    "{character.run_speed_yards()}"
                }
            }
        }
    }
}


#[component]
fn Powers(character: ReadOnlySignal<Character>) -> Element {
    rsx! {
        table {
            class: "stat-table",
            tr {
                class: "heading",
                th { "POWERS, TALENTS, SPECIALIZATIONS" }
            }
            tr {
                td {
                    text_align: "left",
                    height: "10em",
                    vertical_align: "text-top",
                    "TODO"
                }
            }
        }
    }
}


#[component]
fn MeleeWeapons(character: ReadOnlySignal<Character>) -> Element {
    let character = character.read();
    let character = character.deref();

    let mut weapon_training: Vec<_> = character.mechanical_properties.weapon_training.iter().map(|t| t.to_string()).collect();
    weapon_training.sort();
    let weapon_training = weapon_training.join(", ");

    rsx! {
        table {
            class: "stat-table",
            tr {
                class: "heading",
                th { "WEAPON TRAINING" }
            }
            tr {
                td {
                    text_align: "left",
                    "{weapon_training}"
                }
            }
        }
        table {
            class: "stat-table",
            tr {
                class: "heading",
                th { "WEAPON" }
                th {
                    title: "The dice to roll to determine whether you hit.",
                    "ATTACK ROLL"
                }
                th {
                    title: "The dice to roll to determine how much damage you deal.",
                    "DAMAGE ROLL"
                }
            }
            tr {
                td { "TODO" }
                td {
                    title: "TODO",
                    "TODO"
                }
                td {
                    title: "TODO",
                    "TODO"
                }
            }
            tr {
                td { "TODO" }
                td {
                    title: "TODO",
                    "TODO"
                }
                td {
                    title: "TODO",
                    "TODO"
                }
            }
        }
    }
}


#[component]
fn RangeWeapons(character: ReadOnlySignal<Character>) -> Element {
    rsx !{
        table {
            class: "stat-table",
            tr {
                class: "heading",
                th { "WEAPON" }
                th {
                    title: "The dice to roll to determine whether you hit.",
                    "ATTACK ROLL"
                }
                th {
                    title: "The dice to roll to determine how much damage you deal.",
                    "DAMAGE ROLL"
                }
                th {
                    title: "TODO",
                    "SHORT RANGE"
                }
                th {
                    title: "TODO",
                    "LONG RANGE"
                }
                th {
                    title: "TODO",
                    "RELOAD TIME"
                }
            }
            tr {
                td { "TODO" }
                td {
                    title: "TODO",
                    "TODO"
                }
                td {
                    title: "TODO",
                    "TODO"
                }
                td { "TODO" }
                td { "TODO" }
                td { "TODO" }
            }
            tr {
                td { "TODO" }
                td {
                    title: "TODO",
                    "TODO"
                }
                td {
                    title: "TODO",
                    "TODO"
                }
                td { "TODO" }
                td { "TODO" }
                td { "TODO" }
            }
        }
    }
}


fn format_value_modifiers(value: &fage2e::Value) -> String {
    let mut details = if let Some(override_) = value.modifiers.override_ {
        format!("Base: {override_} (overridden by todo)")
    } else {
        format!("Base: {}", value.base)
    };
    for additive in value.modifiers.additive.iter() {
        if additive.value >= 0 {
            details.push_str(
                &format!(
                    "\n+ {} ({})",
                    additive.value,
                    format_modifier_source(&additive.source),
                )
            );
        }
        else {
            details.push_str(
                &format!(
                    "\n- {} ({})",
                    -additive.value,
                    format_modifier_source(&additive.source),
                )
            );
        }
    }
    details
}

fn format_modifier_source(source: &fage2e::ModifierSource) -> String {
    match source {
        fage2e::ModifierSource::Ability(ability) => ability.to_string(),
        fage2e::ModifierSource::Focus(focus) => focus.to_string(),
    }
}
