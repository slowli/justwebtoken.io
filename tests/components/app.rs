//! Tests for the root component.

// TODO: test token + no key

use gloo_timers::future::sleep;
use jwt_compact::{alg::SecretBytes, UntrustedToken};
use wasm_bindgen_test::*;
use web_sys::Element;

use std::{collections::HashMap, time::Duration};

use super::{
    assert_no_child, extract_main_value, extract_rows, select_single_element, TestRigBase,
    HS256_KEY, HS256_TOKEN, K256_JWK,
};
use justwebtoken_io::{
    components::{App, AppMessage, AppProperties},
    keys::KeyInstance,
};

type TestRig = TestRigBase<App>;

fn get_decoded_claims(root_element: &Element) -> HashMap<String, Element> {
    let decoded_claims = root_element
        .query_selector("#decoded-claims")
        .unwrap()
        .unwrap();
    extract_rows(&decoded_claims)
}

#[wasm_bindgen_test]
async fn info_alert_is_displayed_by_default() {
    let rig = TestRig::new(AppProperties::default());
    sleep(Duration::ZERO).await; // wait for rig to fully initialize

    assert_no_child(&rig.root_element, "#decoded-claims");
    assert_no_child(&rig.root_element, ".card-alert.border-danger");

    let alert = select_single_element(&rig.root_element, ".card-alert.border-info");
    let alert_text = alert.text_content().unwrap();
    assert!(alert_text.contains("No key / token"), "{alert_text}");
}

#[wasm_bindgen_test]
async fn claims_are_displayed_for_correct_token() {
    let rig = TestRig::new(AppProperties::default());
    let key = KeyInstance::Symmetric(SecretBytes::borrowed(HS256_KEY));
    rig.send_message(AppMessage::new_key(Some(key))).await;
    let token = UntrustedToken::new(HS256_TOKEN).unwrap().into_owned();
    rig.send_message(AppMessage::new_token(Some(token))).await;

    let claims = get_decoded_claims(&rig.root_element);
    assert_eq!(claims.len(), 3, "{:?}", claims);
    let exp = extract_main_value(&claims["Expiration timestamp"]);
    assert_eq!(exp, "2011-03-22 18:43:00 UTC Token has expired");
    let issuer = extract_main_value(&claims["Issuer"]);
    assert_eq!(issuer, "\"joe\"");
}

#[wasm_bindgen_test]
async fn error_is_displayed_for_incorrect_key_type() {
    let rig = TestRig::new(AppProperties::default());
    let key = serde_json::from_str(K256_JWK).unwrap();
    let key = KeyInstance::new(&key).unwrap();
    rig.send_message(AppMessage::new_key(Some(key))).await;
    let token = UntrustedToken::new(HS256_TOKEN).unwrap().into_owned();
    rig.send_message(AppMessage::new_token(Some(token))).await;

    let alert = select_single_element(&rig.root_element, ".card-alert.border-danger");
    let alert_text = alert.text_content().unwrap();
    assert!(alert_text.contains("Error verifying token"), "{alert_text}");
    assert!(
        alert_text.contains("Token algorithm (HS256) differs from expected (ES256K)"),
        "{alert_text}"
    );
    assert!(
        alert_text.contains("Check that the key is appropriate"),
        "{alert_text}"
    );

    // Claims should still be displayed, albeit with the danger alert.
    let claims = get_decoded_claims(&rig.root_element);
    assert_eq!(claims.len(), 3, "{claims:?}");
}
