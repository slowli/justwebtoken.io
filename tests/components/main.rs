//! Tests related to components.

use wasm_bindgen::JsCast;
use wasm_bindgen_test::wasm_bindgen_test_configure;
use yew::{web_sys::Element, Component, ComponentLink};

use std::collections::HashMap;

mod key_input;
mod token_input;

wasm_bindgen_test_configure!(run_in_browser);

struct TestRigBase<C: Component> {
    root_element: Element,
    component: ComponentLink<C>,
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
        yew::initialize();
        let document = yew::utils::document();
        let div = document.create_element("div").unwrap();
        document.body().unwrap().append_with_node_1(&div).unwrap();

        let component = yew::App::<C>::new().mount_with_props(div.clone(), props);

        Self {
            root_element: div,
            component,
        }
    }

    fn send_message(&self, message: C::Message) {
        self.component.send_message(message);
    }
}

fn select_elements(root: &Element, selector: &str) -> impl Iterator<Item = Element> {
    let nodes = root
        .query_selector_all(selector)
        .unwrap_or_else(|e| panic!("Querying elements `{}` failed: {:?}", selector, e));

    (0..nodes.length()).filter_map(move |i| nodes.get(i).unwrap().dyn_into::<Element>().ok())
}

/// Extracts main value from a value column.
fn extract_main_value(value_column: &Element) -> String {
    let main_value_element = value_column.first_child().expect("no main value");
    main_value_element.text_content().unwrap()
}

/// Extracts `.invalid-feedback` from an element.
fn extract_feedback(element: &Element) -> String {
    let feedback = element
        .query_selector(".invalid-feedback")
        .unwrap()
        .expect("no invalid feedback");
    feedback.text_content().unwrap()
}

fn extract_rows(element: &Element) -> HashMap<String, Element> {
    select_elements(element, ".row")
        .map(|row| {
            let label = row.query_selector("label").unwrap().expect("no label");
            let label = label.text_content().expect("no text in label");
            let value = row.last_child().expect("no value column in row");
            let value = value
                .dyn_into::<Element>()
                .expect("value column is not an element");
            (label, value)
        })
        .collect()
}
