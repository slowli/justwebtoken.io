//! Tests for `<KeyInput />` component.

// TODO: test key usage warning

use assert_matches::assert_matches;
use wasm_bindgen_test::*;
use yew::{web_sys::Element, Callback};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use justwebtoken_io::{
    components::key_input::{KeyInput, KeyInputMessage, KeyInputProperties},
    key_instance::KeyInstance,
};

use super::{extract_feedback, extract_main_value, extract_rows, TestRigBase, K256_JWK};

struct TestRig {
    base: TestRigBase<KeyInput>,
    received_key: Rc<RefCell<Option<KeyInstance>>>,
}

impl TestRig {
    fn new() -> Self {
        let received_key = Rc::new(RefCell::new(None));
        let received_key_ = Rc::clone(&received_key);
        let props = KeyInputProperties {
            onchange: Callback::from(move |key| {
                *received_key_.borrow_mut() = key;
            }),
        };

        Self {
            base: TestRigBase::new(props),
            received_key,
        }
    }

    fn take_received_key(&self) -> KeyInstance {
        self.received_key
            .borrow_mut()
            .take()
            .expect("No key received")
    }

    fn assert_no_received_key(&self) {
        if let Some(key) = &*self.received_key.borrow() {
            panic!("Unexpected received key: {:?}", key);
        }
    }

    fn rows(&self) -> HashMap<String, Element> {
        extract_rows(&self.base.root_element)
    }
}

#[wasm_bindgen_test]
fn correct_key() {
    const KEY_THUMBPRINT: &str = "WXjRM2dXofF2PGP339yJXhia89VsAQRBMZA5_lWuYFY";

    let rig = TestRig::new();
    rig.base
        .send_message(KeyInputMessage::SetKey(K256_JWK.to_owned()));

    assert_matches!(rig.take_received_key(), KeyInstance::K256(_));

    let rows = rig.rows();
    let key_type = extract_main_value(&rows["Type"]);
    assert_eq!(key_type, "Elliptic curve (secp256k1)");
    let key_thumbprint = extract_main_value(&rows["Thumbprint (SHA-256)"]);
    assert_eq!(key_thumbprint, KEY_THUMBPRINT);
}

#[wasm_bindgen_test]
fn incorrect_key_serialization() {
    let rig = TestRig::new();
    rig.base
        .send_message(KeyInputMessage::SetKey("bogus".to_owned()));

    rig.assert_no_received_key();

    let rows = rig.rows();
    assert!(!rows.contains_key("Type"), "{:?}", rows);

    let feedback = extract_feedback(&rows["Verification key"]);
    assert!(feedback.contains("expected value"), "{}", feedback);
}

/// If the key can be parsed, but has invalid type, both feedback and key attributes
/// should be displayed.
#[wasm_bindgen_test]
fn unsupported_key_type() {
    const KEY: &str = r#"{ "crv": "secp256r1", "kty": "EC", "x": "", "y": "" }"#;

    let rig = TestRig::new();
    rig.base
        .send_message(KeyInputMessage::SetKey(KEY.to_owned()));

    rig.assert_no_received_key();

    let rows = rig.rows();
    let key_type = extract_main_value(&rows["Type"]);
    assert_eq!(key_type, "Elliptic curve (secp256r1)");
    assert!(rows.contains_key("Thumbprint (SHA-256)"));

    let feedback = extract_feedback(&rows["Verification key"]);
    assert!(
        feedback.contains("`crv` has unexpected value"),
        "{}",
        feedback
    );
}
