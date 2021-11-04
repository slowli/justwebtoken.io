//! Application components.

use jwt_compact::{jwk::JsonWebKey, UntrustedToken, ValidationError};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use std::fmt;

mod common;
pub mod key_input;
pub mod token_input;

use self::{
    common::{str_to_html, view_data_row, Alert, ComponentRef},
    key_input::{KeyInput, KeyInputMessage},
    token_input::{TokenInput, TokenInputMessage},
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
        err: ExtendedValidationError,
        /// Claims that could be recovered from the token.
        claims: Option<GenericClaims>,
    },
}

#[derive(Debug)]
enum ExtendedValidationError {
    Err(ValidationError),
    NoKey,
}

impl From<ValidationError> for ExtendedValidationError {
    fn from(err: ValidationError) -> Self {
        Self::Err(err)
    }
}

impl ExtendedValidationError {
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

        Alert::Danger.view(
            "Error verifying token",
            html! {
                <>
                    <p class="card-text">{ err }</p>
                    { if let Some(tip) = tip {
                        html! {
                            <p class="card-text text-muted">
                                <small>{ str_to_html(tip) }</small>
                            </p>
                        }
                    } else {
                        html! {}
                    }}
                </>
            },
        )
    }

    fn view_no_key_warning() -> Html {
        Alert::Warning.view(
            "Cannot verify integrity",
            html! {
                <p class="card-text">
                    { "…since no valid verifying key is provided." }
                </p>
            },
        )
    }

    fn view(&self) -> Html {
        match self {
            Self::Err(err) => Self::view_err(err),
            Self::NoKey => Self::view_no_key_warning(),
        }
    }
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
        let token = if let Some(token) = &self.token {
            token
        } else {
            self.result = TokenResult::None;
            return;
        };

        let key = if let Some(key) = &self.key {
            key
        } else {
            let claims = token
                .deserialize_claims_unchecked::<serde_json::Value>()
                .ok();
            self.result = TokenResult::Err {
                err: ExtendedValidationError::NoKey,
                claims,
            };
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
                TokenResult::Err {
                    err: err.into(),
                    claims,
                }
            }
        };
    }
}

#[derive(Debug)]
pub enum AppMessage {
    SetKey(Option<Box<KeyInstance>>),
    SetToken(Option<Box<UntrustedToken<'static>>>),
    RandomToken,
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
    key_input: ComponentRef<KeyInput>,
    token_input: ComponentRef<TokenInput>,
    state: AppState,
}

impl App {
    fn view_claims(claims: &GenericClaims, err: Option<&ExtendedValidationError>) -> Html {
        html! {
            <>
                { if let Some(err) = err {
                    err.view()
                } else {
                    html! {}
                }}
                <div class="d-flex flex-row mb-3">
                    <h3 id="claims" class="mb-0 me-5">{ "Claims" }</h3>
                    { Self::view_claims_nav() }
                </div>
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
        claim.with_html_value(value).view_as_claim(field_name)
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
                    <button
                        type="button"
                        title="Copy raw claims to clipboard"
                        data-clipboard-target="#raw-claims-content"
                        class="btn btn-sm btn-outline-primary btn-copy">
                        { "Copy" }
                    </button>
                </div>
                <pre><code id="raw-claims-content">{ &serialized }</code></pre>
            </div>
        }
    }

    fn view_custom_claim(field_name: &str, value: &serde_json::Value) -> Html {
        let value_str = serde_json::to_string(value).unwrap();
        StandardClaim::get(field_name).map_or_else(
            || Self::view_unknown_claim(field_name, &value_str),
            |claim| Self::view_claim(field_name, claim, &value_str, true),
        )
    }

    fn view_no_inputs_hint(&self) -> Html {
        Alert::Info.view(
            "No key / token",
            html! {
                <>
                    <p class="card-text">
                        { "Provide valid key and token in the inputs above to \
                           start verification." }
                    </p>
                    <button
                        type="button"
                        class="btn btn-info"
                        title="This will also generate a symmetric verifying key"
                        onclick=self.link.callback(|_| AppMessage::RandomToken) >
                        { "Generate random token" }
                    </button>
                </>
            },
        )
    }

    fn generate_random_token(&self) {
        let key = KeyInstance::random_key();
        let token = KeyInstance::random_token(&key);
        let jwk = serde_json::to_string(&JsonWebKey::from(&key)).expect("Cannot serialize key");
        self.key_input.send_message(KeyInputMessage::SetKey(jwk));
        self.token_input
            .send_message(TokenInputMessage::SetToken(token));
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            key_input: ComponentRef::default(),
            token_input: ComponentRef::default(),
            state: AppState::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            AppMessage::SetKey(key) => {
                self.state.key = key.map(|boxed| *boxed);
                self.state.update();
            }
            AppMessage::SetToken(token) => {
                self.state.token = token.map(|boxed| *boxed);
                self.state.update();
            }
            AppMessage::RandomToken => {
                self.generate_random_token();
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <form class="mb-4">
                    <div class="mb-3">
                        <KeyInput
                            component_ref=self.key_input.clone()
                            onchange=self.link.callback(AppMessage::new_key) />
                    </div>
                    <TokenInput
                        component_ref=self.token_input.clone()
                        onchange=self.link.callback(AppMessage::new_token) />
                </form>

                { match &self.state.result {
                    TokenResult::Ok(token) => Self::view_claims(token.claims(), None),
                    TokenResult::Err { err, claims: Some(claims) } =>
                        Self::view_claims(claims, Some(err)),
                    TokenResult::Err { err, claims: None } => err.view(),
                    TokenResult::None => self.view_no_inputs_hint(),
                }}
            </>
        }
    }
}
