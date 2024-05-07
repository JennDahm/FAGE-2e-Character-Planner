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
