use dioxus::prelude::*;

use fage2e;

#[component]
pub fn SelectName(mut name: Signal<fage2e::SelectName>) -> Element {
    rsx! {
        "Enter name: "
        input {
            r#type: "text",
            oninput: move |event| { (*name.write()).name = event.value() },
            value: "{name().name}"
        }
    }
}
