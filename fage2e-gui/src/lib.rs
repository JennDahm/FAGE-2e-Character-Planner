use dioxus::prelude::*;
use tracing::info;

use fage2e::*;

mod character_sheet;

use character_sheet::CharacterSheet;

#[allow(non_snake_case)]
pub fn App() -> Element {
    let base_character = use_signal(|| Character::new());
    let level1 = use_signal(|| Level1::default());
    let character = use_memo(move || {
        let mut character = base_character();
        let _ = level1().apply_all(&mut character);
        character
    });

    rsx! {
        div {
            class: "level-selector",
            button { "Level 1" }
            button { "Level 2" }
            button { "Level 3" }
        }
        div {
            class: "customizer-bar",
            Level1Customizer { level1 }
        }
        CharacterSheet { character }
    }
}

#[component]
fn Level1Customizer(mut level1: Signal<Level1>) -> Element {
    let name = use_signal(move || {
        (*level1.write()).name.clone()
    });
    use_effect(move || {
        let mut level1 = level1.write();
        (*level1).name = name();
        info!("Updated level 1 info");
    });

    rsx! {
        h3 { "Level 1" }
        CharacterNameEntry { name }
    }
}

#[component]
fn CharacterNameEntry(mut name: Signal<SelectName>) -> Element {
    rsx! {
        "Enter name: "
        input {
            r#type: "text",
            oninput: move |event| { (*name.write()).name = event.value() },
            // If we want to do in-line modification:
            // value: "{name().name}"
        }
    }
}
