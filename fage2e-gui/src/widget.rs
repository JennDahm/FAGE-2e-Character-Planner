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
                    class: if *selection.read() == Some(option) {
                        "selected"
                    } else { "unselected" },
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
                    class: if (*selections.read()).contains(&option) {
                        "selected"
                    } else if (*selections.read()).len() >= max_selections() {
                        "disabled"
                    } else {
                        "unselected"
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
