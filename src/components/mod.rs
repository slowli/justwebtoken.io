//! Application components.

use jwt_compact::{UntrustedToken, ValidationError};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use std::fmt;

mod common;
pub mod key_input;
pub mod token_input;

use self::{
    common::{str_to_html, view_data_row},
    key_input::KeyInput,
    token_input::TokenInput,
};
use crate::{
    fields::StandardClaim,
    key_instance::{GenericClaims, GenericToken, KeyInstance},
};

/// Result of token verification.
#[derive(Debug)]
enum TokenResult {
    /// No sufficient inputs to verify the token.
    None,
    /// Token was verified successfully.
    Ok(Box<GenericToken>),
    Err {
        /// Error verifying the token.
        err: ValidationError,
        /// Claims that could be recovered from the token.
        claims: Option<GenericClaims>,
    },
}

#[derive(Debug)]
struct AppState {
    key: Option<KeyInstance>,
    token: Option<UntrustedToken<'static>>,
    result: TokenResult,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            key: None,
            token: None,
            result: TokenResult::None,
        }
    }
}

impl AppState {
    fn update(&mut self) {
        let key = if let Some(key) = &self.key {
            key
        } else {
            self.result = TokenResult::None;
            return;
        };

        let token = if let Some(token) = &self.token {
            token
        } else {
            self.result = TokenResult::None;
            return;
        };

        self.result = match key.verify_token(token) {
            Ok(token) => TokenResult::Ok(Box::new(token)),
            Err(err) => {
                let claims = if matches!(err, ValidationError::MalformedClaims(_)) {
                    // No sense to try deserializing claims again.
                    None
                } else {
                    token
                        .deserialize_claims_unchecked::<serde_json::Value>()
                        .ok()
                };
                TokenResult::Err { err, claims }
            }
        };
    }
}

#[derive(Debug)]
pub enum AppMessage {
    SetKey(Option<Box<KeyInstance>>),
    SetToken(Option<Box<UntrustedToken<'static>>>),
}

impl AppMessage {
    pub fn new_key(key: Option<KeyInstance>) -> Self {
        Self::SetKey(key.map(Box::new))
    }

    pub fn new_token(token: Option<UntrustedToken<'static>>) -> Self {
        Self::SetToken(token.map(Box::new))
    }
}

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    state: AppState,
}

impl App {
    fn view_claims(claims: &GenericClaims, err: Option<&ValidationError>) -> Html {
        html! {
            <>
                <div class="d-flex flex-row mb-3">
                    <h3 id="claims" class="mb-0 me-5">{ "Claims" }</h3>
                    { Self::view_claims_nav() }
                </div>
                { if let Some(err) = err {
                    Self::view_err(err)
                } else {
                    html! {}
                }}
                <div class="tab-content">
                    <div
                        class="tab-pane fade show active"
                        id="decoded-claims"
                        role="tabpanel"
                        aria-labelledby="decoded-claims-tab">

                        { Self::view_decoded_claims(claims) }
                    </div>
                    <div
                        class="tab-pane fade"
                        id="raw-claims"
                        role="tabpanel"
                        aria-labelledby="raw-claims-tab">

                        { Self::view_raw_claims(claims) }
                    </div>
                </div>
            </>
        }
    }

    fn view_claims_nav() -> Html {
        html! {
            <nav class="nav nav-pills">
                <button
                    class="nav-link active"
                    id="decoded-claims-tab"
                    data-bs-toggle="tab"
                    data-bs-target="#decoded-claims"
                    type="button"
                    role="tab"
                    aria-controls="home"
                    aria-selected="true">{ "Decoded" }</button>
                <button
                    class="nav-link"
                    id="raw-claims-tab"
                    data-bs-toggle="tab"
                    data-bs-target="#raw-claims"
                    type="button"
                    role="tab"
                    aria-controls="home"
                    aria-selected="false">{ "Raw" }</button>
            </nav>
        }
    }

    fn view_decoded_claims(claims: &GenericClaims) -> Html {
        let custom_claims = claims.custom.as_object().unwrap();
        let custom_claims_view = custom_claims
            .iter()
            .map(|(name, value)| Self::view_custom_claim(name, value))
            .collect::<Html>();

        html! {
            <>
                { if let Some(expiration) = &claims.expiration {
                    Self::view_claim("exp", StandardClaim::by_name("exp"), expiration, false)
                } else {
                    html! {}
                }}
                { if let Some(issued_at) = &claims.issued_at {
                    Self::view_claim("iat", StandardClaim::by_name("iat"), issued_at, false)
                } else {
                    html! {}
                }}
                { if let Some(not_before) = &claims.not_before {
                    Self::view_claim("nbf", StandardClaim::by_name("nbf"), not_before, false)
                } else {
                    html! {}
                }}
                { custom_claims_view }
            </>
        }
    }

    fn view_claim(
        field_name: &str,
        StandardClaim(claim): StandardClaim,
        value: &dyn fmt::Display,
        show_as_code: bool,
    ) -> Html {
        let value = if show_as_code {
            html! { <code>{ value }</code> }
        } else {
            html! { { value } }
        };

        let label = html! {
            <>
                <label>{ claim.name }</label>
                { " " }
                <span
                    class="badge bg-info text-dark"
                    title="Name of the claim field in claims object">
                    { field_name }
                </span>
            </>
        };
        let value = html! {
            <>
                <div class="mb-0">{ value }</div>
                <div class="text-muted small toggled-description"> { claim.description }</div>
            </>
        };
        view_data_row(label, value)
    }

    fn view_unknown_claim(field_name: &str, value: &str) -> Html {
        view_data_row(
            html! { <label>{ field_name }</label> },
            html! { <div class="mb-0"><code>{ value }</code></div> },
        )
    }

    fn view_raw_claims(claims: &GenericClaims) -> Html {
        let serialized = serde_json::to_string_pretty(claims).unwrap();
        html! {
            <div class="code-snippet">
                <div class="code-snippet-panel">
                    <button type="button" class="btn btn-sm btn-outline-primary">
                        { "Copy" }
                    </button>
                </div>
                <pre><code>{ &serialized }</code></pre>
            </div>
        }
    }

    fn view_err(err: &ValidationError) -> Html {
        let tip = match err {
            ValidationError::InvalidSignature | ValidationError::AlgorithmMismatch { .. } => Some(
                "Check that the key is appropriate for token verification. \
                 If the token provides <code>kid</code> header, it can be used to identify \
                 the key, especially if <code>kid</code> it is a key thumbprint.",
            ),
            ValidationError::MalformedSignature(_) => {
                Some("Check that the token is pasted fully into the corresponding input.")
            }
            ValidationError::MalformedClaims(_) => {
                Some("Check that the token is correctly pasted into the corresponding input.")
            }
            _ => None,
        };

        html! {
            <div class="alert alert-danger" role="alert">
                <h4 class="alert-heading">{ "Error verifying token" }</h4>
                <p>{ err }</p>
                { if let Some(tip) = tip {
                    html! {
                        <>
                            <hr/>
                            <p class="mb-0 small">{ str_to_html(tip) }</p>
                        </>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }

    fn view_custom_claim(field_name: &str, value: &serde_json::Value) -> Html {
        let value_str = serde_json::to_string(value).unwrap();
        if let Some(claim) = StandardClaim::get(field_name) {
            Self::view_claim(field_name, claim, &value_str, true)
        } else {
            Self::view_unknown_claim(field_name, &value_str)
        }
    }

    fn view_no_inputs_hint() -> Html {
        html! {
            <div class="alert alert-info" role="alert">
                <h4>{ "No key / token" }</h4>
                <p class="mb-0">
                    { "Provide valid key and token in the inputs above to start verification." }
                </p>
            </div>
        }
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: AppState::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            AppMessage::SetKey(key) => {
                self.state.key = key.map(|boxed| *boxed);
            }
            AppMessage::SetToken(token) => {
                self.state.token = token.map(|boxed| *boxed);
            }
        }
        self.state.update();
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h3 id="inputs">{ "Inputs" }</h3>
                <form class="mb-4">
                    <div class="mb-3">
                        <KeyInput onchange=self.link.callback(AppMessage::new_key) />
                    </div>
                    <TokenInput onchange=self.link.callback(AppMessage::new_token) />
                </form>

                { match &self.state.result {
                    TokenResult::Ok(token) => Self::view_claims(token.claims(), None),
                    TokenResult::Err { err, claims: Some(claims) } =>
                        Self::view_claims(claims, Some(err)),
                    TokenResult::Err { err, claims: None } => Self::view_err(err),
                    TokenResult::None => Self::view_no_inputs_hint(),
                }}
            </>
        }
    }
}
