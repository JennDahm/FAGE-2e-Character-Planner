use dioxus::prelude::*;

use fage2e;

mod advancement;
mod character_sheet;
mod customizer_bar;
mod level;

#[allow(non_snake_case)]
pub fn App() -> Element {
    let character = use_signal(|| fage2e::Character::new());

    use character_sheet::CharacterSheet;
    use customizer_bar::CustomizerBar;

    rsx! {
        CustomizerBar { character },
        CharacterSheet { character }
    }
}
