#![recursion_limit = "512"]
// Linter settings.
#![warn(missing_debug_implementations, bare_trait_objects)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::non_ascii_literal,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unused_unit
)]

use wasm_bindgen::prelude::*;
use yew::{AppHandle, Renderer};

// Modules are public for the sake of integration testing.
pub mod components;
mod fields;
pub mod keys;
mod rng;

use crate::components::{App, AppMessage, AppProperties};

#[wasm_bindgen]
#[derive(Debug)]
pub struct AppLink {
    inner: AppHandle<App>,
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
    let window = web_sys::window().expect_throw("no Window");
    let document = window.document().expect_throw("no Document");
    let element = document
        .query_selector("#app-root")
        .expect_throw("cannot get app root node")
        .expect_throw("cannot unwrap body node");
    let app = Renderer::<App>::with_root_and_props(element, AppProperties { save }).render();
    AppLink { inner: app }
}
