//! Tests related to components.

use const_decoder::Decoder;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_test::wasm_bindgen_test_configure;
use web_sys::Element;
use yew::{AppHandle, Component};

use std::collections::HashMap;

mod app;
mod key_input;
mod token_input;

wasm_bindgen_test_configure!(run_in_browser);

const HS256_TOKEN: &str = "eyJ0eXAiOiJKV1QiLA0KICJhbGciOiJIUzI1NiJ9.\
    eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly\
    9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ.dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const HS256_KEY: &[u8] = &Decoder::Base64Url.decode::<64>(
    b"AyM1SysPpbyDfgZld3umj1qzKObwVMkoqQ-EstJQLr_T-1qS0gZH75aKtMN3Yj0iPS4hcgUuTwjAzZr1Z9CAow",
);
const K256_JWK: &str = r#"
    {
        "crv": "secp256k1",
        "kty": "EC",
        "x": "IMZEVh0rQx-QkffNRvdOtM0eUmlWEs6n9RXLUwd4KTQ",
        "y": "TAWfWF5I1G8CKS0JN0RO2hgPPlzboRsjVIuCfjfYmeI"
    }
"#;

struct TestRigBase<C: Component> {
    root_element: Element,
    component: AppHandle<C>,
}

impl<C: Component> Drop for TestRigBase<C> {
    // Remove the `root_element` from the document.
    fn drop(&mut self) {
        if let Some(parent) = self.root_element.parent_element() {
            if let Err(err) = parent.remove_child(self.root_element.as_ref()) {
                eprintln!("Error disposing root element for test rig: {:?}", err);
            }
        }
    }
}

impl<C: Component> TestRigBase<C> {
    fn new(props: C::Properties) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.create_element("div").unwrap();
        document.body().unwrap().append_with_node_1(&div).unwrap();

        let component = yew::start_app_with_props_in_element::<C>(div.clone(), props);

        Self {
            root_element: div,
            component,
        }
    }

    fn send_message(&self, message: C::Message) {
        self.component.send_message(message);
    }
}

fn assert_no_child(root: &Element, selector: &str) {
    let selected = root.query_selector(selector).unwrap_or_else(|err| {
        panic!("Cannot query `{}` from {:?}: {:?}", selector, root, err);
    });
    if let Some(selected) = selected {
        panic!("Unexpected element `{}`: {:?}", selector, selected);
    }
}

fn select_elements(root: &Element, selector: &str) -> impl Iterator<Item = Element> {
    let nodes = root
        .query_selector_all(selector)
        .unwrap_or_else(|e| panic!("Querying elements `{}` failed: {:?}", selector, e));

    (0..nodes.length()).filter_map(move |i| nodes.get(i).unwrap().dyn_into::<Element>().ok())
}

fn select_single_element(root: &Element, selector: &str) -> Element {
    let mut iter = select_elements(root, selector);
    let first = iter.next();
    let second = iter.next();

    match (first, second) {
        (None, _) => panic!("`{}` did not match any elements in {:?}", selector, root),
        (Some(_), Some(_)) => panic!("`{}` matched multiple elements in {:?}", selector, root),
        (Some(single), None) => single,
    }
}

/// Extracts main value from a value column.
fn extract_main_value(value_column: &Element) -> String {
    let main_value_element = value_column.first_child().expect_throw("no main value");
    main_value_element.text_content().unwrap()
}

/// Extracts `.invalid-feedback` from an element.
fn extract_feedback(element: &Element) -> String {
    let feedback = element
        .query_selector(".invalid-feedback")
        .unwrap_throw()
        .expect_throw("no invalid feedback");
    feedback.text_content().unwrap()
}

fn extract_rows(element: &Element) -> HashMap<String, Element> {
    select_elements(element, ".row")
        .map(|row| {
            let label = row
                .query_selector("label")
                .unwrap_throw()
                .expect_throw("no label");
            let label = label.text_content().expect_throw("no text in label");
            let value = row.last_child().expect_throw("no value column in row");
            let value = value
                .dyn_into::<Element>()
                .expect_throw("value column is not an element");
            (label, value)
        })
        .collect()
}
