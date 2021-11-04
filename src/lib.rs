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
use yew::{utils::document, ComponentLink};

// Modules are public for the sake of integration testing.
pub mod components;
mod fields;
pub mod key_instance;
mod rng;

use crate::components::{App, AppMessage};

#[wasm_bindgen]
#[derive(Debug)]
pub struct AppLink {
    inner: ComponentLink<App>,
}

#[wasm_bindgen]
impl AppLink {
    #[wasm_bindgen(js_name = randomizeToken)]
    pub fn randomize_token(&self) {
        self.inner.send_message(AppMessage::RandomToken);
    }
}

#[wasm_bindgen(js_name = runApp)]
pub fn run_app() -> AppLink {
    yew::initialize();
    let element = document()
        .query_selector("#app-root")
        .expect("cannot get app root node")
        .expect("cannot unwrap body node");
    let app = yew::App::<App>::new().mount(element);
    yew::run_loop();
    AppLink { inner: app }
}
