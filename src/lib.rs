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
pub mod keys;
mod rng;

use crate::components::{App, AppMessage, AppProperties};

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

    #[wasm_bindgen(js_name = setSaveFlag)]
    pub fn set_save_flag(&self, save: bool) {
        self.inner.send_message(AppMessage::SetSaveFlag(save));
    }
}

#[wasm_bindgen(js_name = runApp)]
pub fn run_app(save: bool) -> AppLink {
    yew::initialize();
    let element = document()
        .query_selector("#app-root")
        .expect_throw("cannot get app root node")
        .expect_throw("cannot unwrap body node");
    let app = yew::App::<App>::new().mount_with_props(element, AppProperties { save });
    yew::run_loop();
    AppLink { inner: app }
}
