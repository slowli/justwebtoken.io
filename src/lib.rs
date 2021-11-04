#![recursion_limit = "512"]
// Linter settings.
#![warn(missing_debug_implementations, bare_trait_objects)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::non_ascii_literal,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use wasm_bindgen::prelude::*;
use yew::utils::document;

// Modules are public for the sake of integration testing.
pub mod components;
mod fields;
pub mod key_instance;
mod rng;

use crate::components::App;

#[wasm_bindgen]
pub fn run_app() {
    yew::initialize();
    let element = document()
        .query_selector("#app-root")
        .expect("cannot get app root node")
        .expect("cannot unwrap body node");
    yew::App::<App>::new().mount(element);
    yew::run_loop();
}
