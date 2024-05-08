use dioxus::prelude::*;

use fage2e;
use fage2e::Advancement;

use crate::util::use_local_storage;

/// Component managing the state of the character customizer side-bar.
///
/// Args:
/// * character: A signal to update with the current character settings.
#[component]
pub fn CustomizerBar(mut character: Signal<fage2e::Character>) -> Element
{
    // Construct the signals we need to manage.
    // Note that we are heavily advised against creating signals/hooks in loops or
    // conditionals, so we have to hand-unroll this.

    // First: The signals for each level advancement.
    //
    // For user convenience, we store them in the browser's LocalStorage, but
    // when operating on them within the app, it's more convenient to use a
    // normal signal.
    let mut level1_storage = use_local_storage("level1", fage2e::Level1::default);
    let level1 = use_signal(move || level1_storage.get());
    use_effect(move || level1_storage.set(level1()));
    // TODO: level2, etc.

    // Next: Signals indicating whether each level advancement is ok and/or fully filled out.
    let mut level1_status = use_signal(|| Result::<bool, ()>::Ok(false));
    // TODO: level2, etc.

    // Finally: Signals for the character state at each level.
    let level0_character = use_signal(|| fage2e::Character::new());
    let level1_character = use_memo(move || {
        let mut character = level0_character();
        level1_status.set(level1().apply_all(&mut character));
        character
    });
    // TODO: level2, etc.

    // Set up an effect to update the current character settings based on the currently
    // selected level.
    let character_levels = [
        level1_character,
    ];
    let mut level_select = use_signal(|| 1u8);
    use_effect(move || {
        character.set(match level_select() {
            lvl if lvl == 1 => { character_levels[(lvl - 1) as usize]() }
            // We should never hit this, but just in case...
            _ => { level0_character() }
        });
    });

    use crate::level::Level1;

    rsx! {
        div {
            class: "level-selector",
            for i in 1..=20 {
                span {
                    class: if level_select() == i { "selected" } else { "unselected" },
                    onclick: move |_| { level_select.set(i); },
                    "Level {i}"
                }
            }
        }
        div {
            class: "customizer-bar",
            match level_select() {
                1 => rsx! { Level1 { character: level0_character, level1 } },
                _ => rsx! {
                    h3 { class: "title", "ERROR" }
                    p { "How did you select an invalid level?" }
                },
            }
        }
    }
}
