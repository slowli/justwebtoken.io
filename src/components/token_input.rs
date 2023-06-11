//! Row with the JSON web token input.

use base64ct::{Base64UrlUnpadded, Encoding};
use jwt_compact::{Header, ParseError, Thumbprint, UntrustedToken};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::{classes, html, Callback, Component, Context, Html, Properties};

use super::common::{view_wide_data_row, ComponentRef, SavedStateManager};
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

    fn view(&self) -> Html {
        html! {
            <>
                { Self::ALG_FIELD.with_value(&self.algorithm).view_aux() }
                { if let Some(key_set_url) = &self.header.key_set_url {
                    StandardHeader::by_name("jku").with_code_value(key_set_url).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(key_id) = &self.header.key_id {
                    StandardHeader::by_name("kid").with_code_value(key_id).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(token_type) = &self.header.token_type {
                    StandardHeader::by_name("typ").with_value(token_type).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(cert_url) = &self.header.certificate_url {
                    StandardHeader::by_name("x5u").with_code_value(cert_url).view_aux()
                } else {
                    html!{}
                }}
                { if let Some(Thumbprint::Bytes(thumb)) = &self.header.certificate_thumbprint {
                    let thumb = Base64UrlUnpadded::encode_string(thumb);
                    StandardHeader::by_name("x5t#S256").with_code_value(&thumb).view_aux()
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
    #[prop_or_default]
    pub save: bool,
}

/// Token input + corresponding diagnostic information.
#[derive(Debug)]
pub struct TokenInput {
    state: TokenInputState,
    state_manager: SavedStateManager,
}

#[derive(Debug)]
pub enum TokenInputMessage {
    SetToken(String),
}

impl TokenInputMessage {
    fn token_set(event: &InputEvent) -> Self {
        let target = event.target().expect_throw("no target for token set event");
        let target = target
            .dyn_into::<HtmlTextAreaElement>()
            .expect_throw("unexpected target for token set event");
        Self::SetToken(target.value())
    }
}

impl Component for TokenInput {
    type Message = TokenInputMessage;
    type Properties = TokenInputProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().component_ref.link_with(ctx.link().clone());
        let (state_manager, init_state) =
            SavedStateManager::new(Self::STORAGE_KEY, ctx.props().save);

        let mut this = Self {
            state: TokenInputState::default(),
            state_manager,
        };
        if let Some(token) = init_state {
            this.update(ctx, TokenInputMessage::SetToken(token));
        }
        this
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            TokenInputMessage::SetToken(token) => {
                self.state_manager.save(&token);
                let (new_state, maybe_token) = TokenInputState::new(token);
                self.state = new_state;
                ctx.props().onchange.emit(maybe_token);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        ctx.props().component_ref.link_with(ctx.link().clone());
        self.state_manager.set_save_flag(ctx.props().save);
        self.state_manager.save(&self.state.raw_token);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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

        let link = ctx.link();
        let row = view_wide_data_row(
            html! {
                <label for="token">
                    <strong>{ "Token" }</strong>
                </label>
            },
            html! {
                <>
                    <textarea
                        id="token"
                        class={control_classes}
                        placeholder="JSON web token"
                        autocomplete="off"
                        spellcheck="false"
                        value={self.state.raw_token.clone()}
                        oninput={link.callback(|evt| TokenInputMessage::token_set(&evt))} >
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
    const STORAGE_KEY: &'static str = "jwt__rawToken";

    fn view_parse_err(err: &ParseError) -> Html {
        html! {
            <p class="invalid-feedback mb-1">{ "Error deserializing token: " }{ err }</p>
        }
    }
}
