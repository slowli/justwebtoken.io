#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::utils::document;

pub mod components;
mod fields;
pub mod key_instance;

use crate::components::App;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    let element = document()
        .query_selector("#app-root")
        .expect("cannot get app root node")
        .expect("cannot unwrap body node");
    yew::App::<App>::new().mount(element);
    yew::run_loop();

    Ok(())
}
