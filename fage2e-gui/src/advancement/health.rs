use dioxus::prelude::*;

use fage2e;

#[component]
pub fn DiceBasedHealthAdvancement(
    advancement: Signal<fage2e::DiceBasedHealthAdvancement>, constitution: ReadOnlySignal<i8>
) -> Element {
    let dice = fage2e::DiceBasedHealthAdvancement::dice();
    let calculated = advancement().calculated(constitution());

    use crate::widget::Button;

    rsx! {
        h4 { class: "section-header", "Health Advancement" }
        p {
            class: "label",
            "CON ({constitution()}) + "
            input {
                r#type: "number",
                min: "{dice.min_value()}",
                max: "{dice.max_value()}",
                value: if let Some(roll) = advancement().roll_result {
                    format!("{roll}")
                },
                onchange: move |event| {
                    let val: i32 = event.value().parse().unwrap_or(0);
                    (*advancement.write()).roll_result = if val < dice.min_value() as i32 {
                        None
                    } else if val < dice.max_value() as i32 {
                        Some(val as u8)
                    } else {
                        Some(dice.max_value() as u8)
                    };
                },
            }
            " "
            Button {
                text: "Roll",
                disabled: false,
                onclick: move |_| {
                    (*advancement.write()).roll_result = Some(dice.roll_single());
                },
            }
        }
        p {
            class: "label",
            "Total: "
            match calculated {
                Ok(v) => format!("{v}"),
                Err(v) => format!("{v} (rounded up)"),
            }
        }
    }
}
