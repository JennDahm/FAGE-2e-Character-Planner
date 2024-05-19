use dioxus::prelude::*;

use fage2e;

mod ability_determination;
mod ancestry;
mod class;
mod class_selection;
mod health;
mod initial_weapon_group;

pub use ability_determination::*;
pub use ancestry::*;
pub use class_selection::*;
pub use health::*;
pub use initial_weapon_group::*;

#[component]
pub fn SelectName(mut name: Signal<fage2e::SelectName>) -> Element {
    rsx! {
        h4 { class: "section-header", "Character Name" }
        input {
            r#type: "text",
            // For live update:
            // oninput: move |event| { (*name.write()).name = event.value() },
            // Only when pressing enter/changing focus:
            onchange: move |event| { (*name.write()).name = event.value() },
            value: "{name().name}"
        }
    }
}
