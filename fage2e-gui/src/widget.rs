use dioxus::prelude::*;

/// A widget for selecting one (or none) among several options.
#[component]
pub fn Selector<T: Copy + PartialEq + std::fmt::Display + 'static>(
    options: ReadOnlySignal<Vec<T>>, selection: Signal<Option<T>>
) -> Element
{
    rsx! {
        div {
            class: "selector",
            for option in options() {
                span {
                    class: "pressable",
                    class: if *selection.read() == Some(option) {
                        "pressed"
                    } else { "unpressed" },
                    onclick: move |_| {
                        if *selection.read() == Some(option) {
                            selection.set(None);
                        }
                        else {
                            selection.set(Some(option));
                        }
                    },
                    "{option}"
                }
            }
        }
    }
}

/// A widget for selecting a number of options.
#[component]
pub fn MultiSelector<T: Copy + PartialEq + std::fmt::Display + 'static>(
    options: ReadOnlySignal<Vec<T>>, selections: Signal<Vec<T>>, max_selections: ReadOnlySignal<usize>
) -> Element
{
    rsx! {
        div {
            class: "selector",
            for option in options() {
                span {
                    class: "pressable",
                    class: if (*selections.read()).contains(&option) {
                        "pressed"
                    } else if (*selections.read()).len() >= max_selections() {
                        "disabled"
                    } else {
                        "unpressed"
                    },
                    onclick: move |_| {
                        let idx = (*selections.read()).iter().position(|&x| x == option);
                        if let Some(idx) = idx {
                            (*selections.write()).remove(idx);
                        } else if (*selections.read()).len() < max_selections() {
                            (*selections.write()).push(option);
                        }
                    },
                    "{option}"
                }
            }
        }
    }
}

/// A widget for selecting a number of options, where some of those options may
/// be disabled.
#[component]
pub fn DisableableMultiSelector<T: Copy + PartialEq + std::fmt::Display + 'static>(
    options: ReadOnlySignal<Vec<T>>,
    disabled_options: ReadOnlySignal<Vec<T>>,
    selections: Signal<Vec<T>>,
    max_selections: ReadOnlySignal<usize>,
) -> Element
{
    rsx! {
        div {
            class: "selector",
            for option in options() {
                span {
                    class: "pressable",
                    class: if (*selections.read()).contains(&option) {
                        "pressed"
                    } else if (*disabled_options.read()).contains(&option) {
                        "disabled"
                    } else if (*selections.read()).len() >= max_selections() {
                        "disabled"
                    } else {
                        "unpressed"
                    },
                    onclick: move |_| {
                        let idx = (*selections.read()).iter().position(|&x| x == option);
                        if let Some(idx) = idx {
                            (*selections.write()).remove(idx);
                        }
                        else if (*disabled_options.read()).contains(&option) {
                            // Do nothing if the option is disabled.
                        }
                        else if (*selections.read()).len() < max_selections() {
                            (*selections.write()).push(option);
                        }
                    },
                    "{option}"
                }
            }
        }
    }
}

/// A reusable button widget.
///
/// This button does not latch.
#[component]
pub fn Button(text: String, disabled: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let mut pressed = use_signal(|| false);
    rsx! {
        span {
            class: "pressable",
            class: if disabled {
                "disabled"
            } else if pressed() {
                "pressed"
            } else {
                "unpressed"
            },
            onmousedown: move |_| { pressed.set(true); },
            onmouseup: move |_| { pressed.set(false); },
            onmouseleave: move |_| { pressed.set(false); },
            onclick: move |event| { if !disabled { onclick(event); } },
            "{text}"
        }
    }
}
