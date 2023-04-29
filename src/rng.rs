//! Simpler CSPRNG binding to get rid of dynamic `require` calls, which (reasonably)
//! irritate Webpack.

#![allow(clippy::no_mangle_with_rust_abi)] // emitted by `register_custom_getrandom!`

use getrandom::{register_custom_getrandom, Error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getRandomValues, js_namespace = crypto)]
    fn random_bytes(dest: &mut [u8]);
}

#[allow(clippy::unnecessary_wraps)] // this specific signature is required by `getrandom`
fn random_bytes_shim(dest: &mut [u8]) -> Result<(), Error> {
    random_bytes(dest);
    Ok(())
}

register_custom_getrandom!(random_bytes_shim);
