//! Tests for `<TokenInput />` component.

use jwt_compact::UntrustedToken;
use wasm_bindgen_test::*;
use yew::{web_sys::Element, Callback};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use justwebtoken_io::components::token_input::{
    TokenInput, TokenInputMessage, TokenInputProperties,
};

use super::{extract_feedback, extract_main_value, extract_rows, TestRigBase};

struct TestRig {
    base: TestRigBase<TokenInput>,
    received_token: Rc<RefCell<Option<UntrustedToken<'static>>>>,
}

impl TestRig {
    fn new() -> Self {
        let received_token = Rc::new(RefCell::new(None));
        let received_token_ = Rc::clone(&received_token);
        let props = TokenInputProperties {
            onchange: Callback::from(move |key| {
                *received_token_.borrow_mut() = Some(key);
            }),
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
            .expect("No token received")
    }

    fn assert_no_received_token(&self) {
        if let Some(token) = &*self.received_token.borrow() {
            panic!("Unexpected received token: {:?}", token);
        }
    }

    fn rows(&self) -> HashMap<String, Element> {
        extract_rows(&self.base.root_element)
    }
}

const HS256_TOKEN: &str = "eyJ0eXAiOiJKV1QiLA0KICJhbGciOiJIUzI1NiJ9.\
    eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly\
    9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ.dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

#[wasm_bindgen_test]
fn correct_token() {
    let rig = TestRig::new();
    rig.base
        .send_message(TokenInputMessage::SetToken(HS256_TOKEN.to_owned()));

    let received_token = rig.take_received_token();
    assert_eq!(received_token.algorithm(), "HS256");

    let rows = rig.rows();
    let alg = extract_main_value(&rows["Algorithm"]);
    assert_eq!(alg, "HS256");
    let typ = extract_main_value(&rows["Type"]);
    assert_eq!(typ, "JWT");
}

#[wasm_bindgen_test]
fn incorrect_token_serialization() {
    let rig = TestRig::new();
    rig.base
        .send_message(TokenInputMessage::SetToken("!!!".to_owned()));

    rig.assert_no_received_token();

    let rows = rig.rows();
    assert!(!rows.contains_key("Algorithm"), "{:?}", rows);

    let feedback = extract_feedback(&rows["Token"]);
    assert!(
        feedback.contains("Error deserializing token"),
        "{}",
        feedback
    );
}
