//! Application.

// FIXME: deserialize & display claims on empty key.

use jwt_compact::{UntrustedToken, ValidationError};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use std::fmt;

mod common;
pub mod key_input;
mod token_input;

use self::{key_input::KeyInput, token_input::TokenInput};
use crate::{
    fields::StandardClaim,
    key_instance::{GenericClaims, GenericToken, KeyInstance},
};

#[derive(Debug)]
struct AppState {
    key: Option<KeyInstance>,
    token: Option<UntrustedToken<'static>>,
    result: Result<Option<GenericToken>, ValidationError>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            key: None,
            token: None,
            result: Ok(None),
        }
    }
}

impl AppState {
    fn update(&mut self) {
        let key = if let Some(key) = &self.key {
            key
        } else {
            return;
        };

        let token = if let Some(token) = &self.token {
            token
        } else {
            return;
        };

        self.result = key.verify_token(token).map(Some);
    }
}

#[derive(Debug)]
pub enum AppMessage {
    SetKey(Box<KeyInstance>),
    SetToken(Box<UntrustedToken<'static>>),
}

impl AppMessage {
    fn new_key(key: KeyInstance) -> Self {
        Self::SetKey(Box::new(key))
    }
}

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    state: AppState,
}

impl App {
    fn view_claims(token: &GenericToken) -> Html {
        html! {
            <>
                <div class="d-flex flex-row mb-3">
                    <h2 class="mb-0 me-5">{ "Claims" }</h2>
                    { Self::view_claims_nav() }
                </div>
                <div class="tab-content">
                    <div
                        class="tab-pane fade show active"
                        id="decoded-claims"
                        role="tabpanel"
                        aria-labelledby="decoded-claims-tab">

                        { Self::view_decoded_claims(token.claims()) }
                    </div>
                    <div
                        class="tab-pane fade"
                        id="raw-claims"
                        role="tabpanel"
                        aria-labelledby="raw-claims-tab">

                        { Self::view_raw_claims(token.claims()) }
                    </div>
                </div>
            </>
        }
    }

    fn view_claims_nav() -> Html {
        html! {
            <nav class="nav nav-pills mb-3">
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

        html! {
            <div class="mb-2 row">
                <div class="col-3">
                    { claim.name }
                    { " " }
                    <span
                        class="badge bg-info text-dark"
                        title="Name of the claim field in claims object">
                        { field_name }
                    </span>
                </div>
                <div class="col-9">
                    <div class="mb-0">{ value }</div>
                    <div class="text-muted small"> { claim.description }</div>
                </div>
            </div>
        }
    }

    fn view_unknown_claim(field_name: &str, value: &str) -> Html {
        html! {
            <div class="mb-2 row">
                <div class="col-3">
                    { field_name }
                </div>
                <div class="col-9">
                    <div class="mb-0"><code>{ value }</code></div>
                </div>
            </div>
        }
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

    // FIXME: render properly
    fn view_err(err: &ValidationError) -> Html {
        html! {
            { err }
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
                self.state.key = Some(*key);
            }
            AppMessage::SetToken(token) => {
                self.state.token = Some(*token);
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
                <h2>{ "Verification inputs" }</h2>
                <form class="mb-5">
                    <div class="mb-3">
                        <KeyInput onchange=self.link.callback(AppMessage::new_key) />
                    </div>
                    <TokenInput />
                </form>

                <hr class="mb-4" />

                { match &self.state.result {
                    Ok(Some(token)) => Self::view_claims(token),
                    Err(err) => Self::view_err(err),
                    _ => html!{}
                }}
            </>
        }
    }
}
