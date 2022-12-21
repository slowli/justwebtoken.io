//! Tests for `<TokenInput />` component.

// TODO: test state persistence

use jwt_compact::UntrustedToken;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_test::*;
use web_sys::Element;
use yew::Callback;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{extract_feedback, extract_main_value, extract_rows, TestRigBase, HS256_TOKEN};
use justwebtoken_io::components::token_input::{
    TokenInput, TokenInputMessage, TokenInputProperties,
};

struct TestRig {
    base: TestRigBase<TokenInput>,
    received_token: Rc<RefCell<Option<UntrustedToken<'static>>>>,
}

impl TestRig {
    fn new() -> Self {
        let received_token = Rc::new(RefCell::new(None));
        let received_token_ = Rc::clone(&received_token);
        let props = TokenInputProperties {
            onchange: Callback::from(move |token| {
                *received_token_.borrow_mut() = token;
            }),
            ..TokenInputProperties::default()
        };

        Self {
            base: TestRigBase::new(props),
            received_token,
        }
    }

    fn take_received_token(&self) -> UntrustedToken<'static> {
        self.received_token
            .borrow_mut()
            .take()
            .expect_throw("no token received")
    }

    fn assert_no_received_token(&self) {
        if let Some(token) = &*self.received_token.borrow() {
            panic!("Unexpected received token: {token:?}");
        }
    }

    fn rows(&self) -> HashMap<String, Element> {
        extract_rows(&self.base.root_element)
    }
}

#[wasm_bindgen_test]
async fn correct_token() {
    let rig = TestRig::new();
    rig.base
        .send_message(TokenInputMessage::SetToken(HS256_TOKEN.to_owned()))
        .await;

    let received_token = rig.take_received_token();
    assert_eq!(received_token.algorithm(), "HS256");

    let rows = rig.rows();
    let alg = extract_main_value(&rows["Algorithm"]);
    assert_eq!(alg, "HS256");
    let typ = extract_main_value(&rows["Type"]);
    assert_eq!(typ, "JWT");
}

#[wasm_bindgen_test]
async fn incorrect_token_serialization() {
    let rig = TestRig::new();
    rig.base
        .send_message(TokenInputMessage::SetToken("!!!".to_owned()))
        .await;

    rig.assert_no_received_token();

    let rows = rig.rows();
    assert!(!rows.contains_key("Algorithm"), "{rows:?}");

    let feedback = extract_feedback(&rows["Token"]);
    assert!(feedback.contains("Error deserializing token"), "{feedback}");
}
