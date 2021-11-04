//! Row with the JSON web token input.

use base64ct::{Base64UrlUnpadded, Encoding};
use jwt_compact::{Header, ParseError, UntrustedToken};
use yew::{
    classes, html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender,
};

use super::common::{view_data_row, ComponentRef};
use crate::fields::{Field, StandardHeader};

#[derive(Debug)]
struct TokenInputState {
    raw_token: String,
    parse_result: Result<Option<ParsedHeader>, ParseError>,
}

impl Default for TokenInputState {
    fn default() -> Self {
        Self {
            raw_token: String::new(),
            parse_result: Ok(None),
        }
    }
}

impl TokenInputState {
    fn new(raw_token: String) -> (Self, Option<UntrustedToken<'static>>) {
        let token = UntrustedToken::new(&raw_token).map(UntrustedToken::into_owned);
        let (parse_result, token) = match token {
            Ok(token) => (Ok(Some(ParsedHeader::new(&token))), Some(token)),
            Err(err) => (Err(err), None),
        };

        let this = Self {
            raw_token,
            parse_result,
        };
        (this, token)
    }
}

#[derive(Debug)]
struct ParsedHeader {
    algorithm: String,
    header: Header,
}

impl ParsedHeader {
    const ALG_FIELD: Field = Field {
        name: "Algorithm",
        description: "Integrity algorithm used to secure the token.",
        link: Some("https://datatracker.ietf.org/doc/html/rfc7515#section-4.1.1"),
    };

    fn new(token: &UntrustedToken<'_>) -> Self {
        Self {
            algorithm: token.algorithm().to_owned(),
            header: token.header().clone(),
        }
    }

    // TODO: Describe algorithm (in popover?)
    fn view(&self) -> Html {
        html! {
            <>
                { Self::ALG_FIELD.with_value(&self.algorithm).view_aux() }
                { if let Some(key_set_url) = &self.header.key_set_url {
                    StandardHeader::by_name("jku").with_value(key_set_url).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(key_id) = &self.header.key_id {
                    StandardHeader::by_name("kid").with_value(key_id).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(token_type) = &self.header.token_type {
                    StandardHeader::by_name("typ").with_value(token_type).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(cert_url) = &self.header.certificate_url {
                    StandardHeader::by_name("x5u").with_value(cert_url).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(cert_thumb) = &self.header.certificate_thumbprint {
                    let cert_thumb = Base64UrlUnpadded::encode_string(cert_thumb);
                    StandardHeader::by_name("x5t#S256").with_value(&cert_thumb).view_aux()
                } else {
                    html!{}
                }}
            </>
        }
    }
}

/// Properties for the `TokenInput` component.
#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct TokenInputProperties {
    #[prop_or_default]
    pub component_ref: ComponentRef<TokenInput>,
    #[prop_or_default]
    pub onchange: Callback<Option<UntrustedToken<'static>>>,
}

/// Token input + corresponding diagnostic information.
#[derive(Debug)]
pub struct TokenInput {
    link: ComponentLink<Self>,
    onchange: Callback<Option<UntrustedToken<'static>>>,
    state: TokenInputState,
}

#[derive(Debug)]
pub enum TokenInputMessage {
    SetToken(String),
}

impl Component for TokenInput {
    type Message = TokenInputMessage;
    type Properties = TokenInputProperties;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        properties.component_ref.link_with(link.clone());
        Self {
            link,
            onchange: properties.onchange,
            state: TokenInputState::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            TokenInputMessage::SetToken(token) => {
                let (new_state, maybe_token) = TokenInputState::new(token);
                self.state = new_state;
                self.onchange.emit(maybe_token);
            }
        }
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        properties.component_ref.link_with(self.link.clone());
        self.onchange = properties.onchange;
        false
    }

    fn view(&self) -> Html {
        let parse_res = self.state.parse_result.as_ref();
        let mut control_classes = classes![
            "form-control",
            "mb-1",
            "font-monospace",
            "small",
            "text-break-all"
        ];
        if parse_res.is_err() {
            control_classes.push("is-invalid");
        }

        let row = view_data_row(
            html! {
                <label for="token" class="col-form-label">
                    <strong>{ "Token" }</strong>
                </label>
            },
            html! {
                <>
                    <textarea
                        id="token"
                        class=control_classes
                        placeholder="JSON web token"
                        value=self.state.raw_token.clone()
                        oninput=self.link.callback(move |e: InputData| {
                            TokenInputMessage::SetToken(e.value)
                        }) >
                        { &self.state.raw_token }
                    </textarea>

                    { if let Err(err) = parse_res {
                        Self::view_parse_err(err)
                    } else {
                        html!{}
                    }}
                </>
            },
        );

        html! {
            <>
                { row }
                { if let Ok(Some(header)) = parse_res {
                    header.view()
                } else {
                    html!{}
                }}
            </>
        }
    }
}

impl TokenInput {
    fn view_parse_err(err: &ParseError) -> Html {
        html! {
            <p class="invalid-feedback mb-1">{ "Error deserializing token: " }{ err }</p>
        }
    }
}
