//! Row with the JSON web key input.

// TODO: base64/hex key input + type (symmetric | ed25519 | k256)

use base64ct::{Base64UrlUnpadded, Encoding};
use jwt_compact::jwk::{JsonWebKey, JwkError};
use sha2::Sha256;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::{classes, html, Callback, Component, Context, Html, Properties};

use std::fmt;

use super::common::{view_wide_data_row, ComponentRef, Icon, SavedStateManager};
use crate::{fields::Field, keys::KeyInstance};

/// Key type together with auxiliary information.
#[derive(Debug)]
enum ExtendedKeyType {
    Rsa { bits: usize },
    Symmetric { bytes: usize },
    EllipticCurve { curve_name: String },
}

impl ExtendedKeyType {
    fn new(jwk: &JsonWebKey<'_>) -> Self {
        match jwk {
            JsonWebKey::Rsa { modulus, .. } => Self::Rsa {
                bits: modulus.len() * 8,
            },

            JsonWebKey::EllipticCurve { curve, .. } | JsonWebKey::KeyPair { curve, .. } => {
                Self::EllipticCurve {
                    curve_name: curve.clone().into_owned(),
                }
            }

            JsonWebKey::Symmetric { secret } => Self::Symmetric {
                bytes: secret.len(),
            },

            // The newest version of `jwt-compact` should be used.
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ExtendedKeyType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rsa { bits } => write!(formatter, "RSA ({} bits)", bits),
            Self::EllipticCurve { curve_name } => {
                write!(formatter, "Elliptic curve ({})", curve_name)
            }
            Self::Symmetric { bytes } => write!(formatter, "Symmetric ({} bytes)", bytes),
        }
    }
}

#[derive(Debug)]
struct ParsedKey {
    key_type: ExtendedKeyType,
    is_signing_key: bool,
    sha256_thumbprint: [u8; 32],
    instantiate_result: Result<(), JwkError>,
}

impl ParsedKey {
    const KEY_FIELD: Field = Field {
        name: "Type",
        description: "Different key types may be used with different signing algorithms. \
            The mapping is not 1-to-1, however; e.g., RSA keys can be used with any of 6 \
            <code>RS*</code> and <code>PS*</code> algorithms.",
        link: None,
    };

    const THUMBPRINT_FIELD: Field = Field {
        name: "Thumbprint (SHA-256)",
        description: "As defined in RFC 7638, a key thumbprint is computed by hashing its \
            canonical <abbr title=\"JSON web key\">JWK</abbr> presentation (only necessary fields \
            sorted in alphabetic order). SHA-256 hash function is used for hashing.",
        link: Some("https://tools.ietf.org/html/rfc7638"),
    };

    const KEY_USAGE_FIELD: Field = Field {
        name: "Usage",
        description: "JWK for a signing key is always valid as a JWK for the corresponding \
            verifying key. Thus, it could be mistakenly used instead of a verifying JWK.",
        link: None,
    };

    fn view(&self) -> Html {
        let should_warn =
            self.is_signing_key && !matches!(self.key_type, ExtendedKeyType::Symmetric { .. });
        let thumbprint = Base64UrlUnpadded::encode_string(&self.sha256_thumbprint);

        html! {
            <>
                { if should_warn {
                    Self::view_signing_key_warning()
                } else {
                    html!{}
                }}
                { Self::KEY_FIELD.with_value(&self.key_type).view_aux() }
                { Self::THUMBPRINT_FIELD.with_code_value(&thumbprint).view_aux() }
            </>
        }
    }

    fn view_signing_key_warning() -> Html {
        let usage = html! {
            <>
                <span
                    class="badge bg-warning text-dark me-2"
                    title="Potentially incorrect key usage!">
                    { Icon::Warning.view() }{ " signing" }
                </span>
                <span class="badge bg-secondary me-2">{ "verification" }</span>
            </>
        };
        Self::KEY_USAGE_FIELD.with_html_value(usage).view_aux()
    }
}

#[derive(Debug)]
struct KeyInputState {
    raw_key: String,
    parse_result: Result<Option<ParsedKey>, serde_json::Error>,
}

impl Default for KeyInputState {
    fn default() -> Self {
        Self {
            raw_key: String::new(),
            parse_result: Ok(None),
        }
    }
}

impl KeyInputState {
    fn new(raw_key: String) -> (Self, Option<KeyInstance>) {
        let jwk = match serde_json::from_str::<JsonWebKey<'_>>(&raw_key) {
            Ok(jwk) => jwk,
            Err(err) => {
                let this = Self {
                    raw_key,
                    parse_result: Err(err),
                };
                return (this, None);
            }
        };

        let mut sha256_thumbprint = [0_u8; 32];
        sha256_thumbprint.copy_from_slice(&jwk.thumbprint::<Sha256>());

        let (key_instance, instantiate_result) = match KeyInstance::new(&jwk) {
            Ok(key) => (Some(key), Ok(())),
            Err(err) => (None, Err(err)),
        };

        let this = Self {
            raw_key,
            parse_result: Ok(Some(ParsedKey {
                key_type: ExtendedKeyType::new(&jwk),
                is_signing_key: jwk.is_signing_key(),
                sha256_thumbprint,
                instantiate_result,
            })),
        };
        (this, key_instance)
    }

    fn error(&self) -> Option<&dyn fmt::Display> {
        match &self.parse_result {
            Err(err) => Some(err),
            Ok(Some(key)) => key
                .instantiate_result
                .as_ref()
                .err()
                .map(|err| err as &dyn fmt::Display),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum KeyInputMessage {
    SetKey(String),
}

impl KeyInputMessage {
    fn key_set(event: &InputEvent) -> Self {
        let target = event.target().expect_throw("no target for key set event");
        let target = target
            .dyn_into::<HtmlTextAreaElement>()
            .expect_throw("unexpected target for key set event");
        Self::SetKey(target.value())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct KeyInputProperties {
    #[prop_or_default]
    pub component_ref: ComponentRef<KeyInput>,
    #[prop_or_default]
    pub onchange: Callback<Option<KeyInstance>>,
    #[prop_or_default]
    pub save: bool,
}

#[derive(Debug)]
pub struct KeyInput {
    state: KeyInputState,
    state_manager: SavedStateManager,
}

impl Component for KeyInput {
    type Message = KeyInputMessage;
    type Properties = KeyInputProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().component_ref.link_with(ctx.link().clone());

        let (state_manager, init_state) =
            SavedStateManager::new(Self::STORAGE_KEY, ctx.props().save);

        let mut this = Self {
            state: KeyInputState::default(),
            state_manager,
        };

        if let Some(key) = init_state {
            this.update(ctx, KeyInputMessage::SetKey(key));
        }
        this
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            KeyInputMessage::SetKey(key) => {
                self.state_manager.save(&key);
                let (new_state, maybe_key) = KeyInputState::new(key);
                self.state = new_state;
                ctx.props().onchange.emit(maybe_key);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.props().component_ref.link_with(ctx.link().clone());

        self.state_manager.set_save_flag(ctx.props().save);
        self.state_manager.save(&self.state.raw_key);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut control_classes = classes![
            "form-control",
            "mb-1",
            "font-monospace",
            "small",
            "text-break-all"
        ];
        let err = self.state.error();
        if err.is_some() {
            control_classes.push("is-invalid");
        }

        let link = ctx.link();
        let row = view_wide_data_row(
            html! {
                <label for="key">
                    <strong>{ "Verifying key" }</strong>
                </label>
            },
            html! {
                <>
                    <textarea
                        id="key"
                        class={control_classes}
                        placeholder="Encoded key"
                        autocomplete="off"
                        spellcheck="false"
                        value={self.state.raw_key.clone()}
                        oninput={link.callback(|evt| KeyInputMessage::key_set(&evt))} >
                        { &self.state.raw_key }
                    </textarea>

                    { if let Some(err) = err {
                        Self::view_err(err)
                    } else {
                        html!{}
                    }}

                    <div class="form-text">
                        { "A key should be provided in the " }
                        <a href="https://www.rfc-editor.org/rfc/rfc7517.html">{ "JSON Web Key" }</a>
                        { " format, that is, as a JSON object." }
                    </div>
                </>
            },
        );

        html! {
            <>
                { row }
                { if let Ok(Some(key)) = &self.state.parse_result {
                    key.view()
                } else {
                    html!{}
                }}
            </>
        }
    }
}

impl KeyInput {
    const STORAGE_KEY: &'static str = "jwt__rawKey";

    fn view_err(err: &dyn fmt::Display) -> Html {
        html! {
            <p class="invalid-feedback mb-1">{ err }</p>
        }
    }
}
