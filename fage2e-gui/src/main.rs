use dioxus::prelude::*;
use tracing::Level;

use fage2e_gui::App;


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
