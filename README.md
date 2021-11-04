# Just Web Tokens

[![License: Apache-2.0][license-image]][license-url] 

[license-image]: https://img.shields.io/github/license/slowli/justwebtoken.io.svg
[license-url]: https://github.com/slowli/justwebtoken.io/blob/master/LICENSE

Web app for JSON Web Token verification also providing a gentle overview of the JWT tech.
Dynamic logic is built with the Rust / WASM toolchain and [Yew]. Packaged with [Webpack]. Styled using [Bootstrap].
JWT verification is powered by the [`jwt-compact`] Rust library with pure-Rust crypto backends for [RSA][`rsa`],
[Ed25519][`ed25519-compact`] and [secp256k1][`k256`].

## Running locally

You will need to install a Node / npm toolchain (preferably via a manager like [`nvm`])
and a Rust toolchain (preferably via [`rustup`]). Both toolchains should be recent; i.e., Node 16-LTS
and Rust 1.51+. You should also install [`wasm-pack`].

To serve the app locally with the Webpack dev server, run

```shell
npm start
```

## Testing

To run tests, use `npm test`.
Be aware that this command requires specifying browsers used for testing as flags
(e.g., `-- --firefox`).

Consult [`package.json`](package.json) for the full list of linting and testing commands.
Note that Rust-related linting requires additional components (`fmt` and `clippy`) installed as a part
of the relevant toolchain.

## License

Licensed under [Apache-2.0 license](LICENSE).

[Yew]: https://yew.rs/
[Webpack]: https://webpack.js.org/
[Bootstrap]: https://getbootstrap.com/
[`rsa`]: https://crates.io/crates/rsa
[`ed25519-compact`]: https://crates.io/crates/ed25519-compact
[`k256`]: https://crates.io/crates/k256
[`nvm`]: https://github.com/creationix/nvm
[`rustup`]: https://rustup.rs/
[`wasm-pack`]: https://rustwasm.github.io/wasm-pack/installer/
[`jwt-compact`]: https://crates.io/crates/jwt-compact