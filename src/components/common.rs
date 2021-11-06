//! Common reused components.

use wasm_bindgen::UnwrapThrowExt;
use yew::{classes, html, web_sys, Component, ComponentLink, Html};

use std::{cell::RefCell, rc::Rc};

use crate::fields::FieldWithValue;

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Link,
    Warning,
    Info,
}

impl Icon {
    fn icon_class(self) -> &'static str {
        match self {
            Self::Link => "bi-book",
            Self::Warning => "bi-exclamation-diamond",
            Self::Info => "bi-info-circle",
        }
    }

    pub fn view(self) -> Html {
        html! { <i class=classes!("bi", self.icon_class())></i> }
    }
}

impl FieldWithValue {
    pub fn view_aux(self) -> Html {
        let field = &self.field;
        view_data_row(
            html! {
                <div class="ps-md-2">
                    <label class="text-decoration-underline">{ field.name }</label>
                    { if let Some(link) = field.link {
                        Self::view_link(link)
                    } else {
                        html!{}
                    }}
                </div>
            },
            html! {
                <>
                    <div>{ self.value }</div>
                    <div class="text-muted small toggled-description">
                        { str_to_html(field.description) }
                    </div>
                </>
            },
        )
    }

    fn view_link(link: &'static str) -> Html {
        html! {
            <>
                { "\u{00a0}" } // non-breakable space
                <a href=link
                    target="_blank"
                    class="text-decoration-none"
                    title="View field definition">
                    { Icon::Link.view() }
                </a>
            </>
        }
    }

    pub fn view_as_claim(self, original_name: &str) -> Html {
        let field = &self.field;
        view_data_row(
            html! {
                <>
                    <label><strong>{ field.name }</strong></label>
                    { " – " }
                    <abbr title="Name of the claim field in claims object">
                        { original_name }
                    </abbr>
                    { if let Some(link) = field.link {
                        Self::view_link(link)
                    } else {
                        html!{}
                    }}
                </>
            },
            html! {
                <>
                    <div class="mb-0">{ self.value }</div>
                    <div class="text-muted small toggled-description">
                        { str_to_html(field.description) }
                    </div>
                </>
            },
        )
    }
}

pub fn str_to_html(html_str: &str) -> Html {
    let div = yew::utils::document().create_element("div").unwrap();
    div.set_inner_html(html_str);
    Html::VRef(div.into())
}

pub fn view_data_row(label: Html, value: Html) -> Html {
    html! {
        <div class="row mb-1">
            <div class="col-md-4 col-lg-3">{ label }</div>
            <div class="col-md-8 col-lg-9">{ value }</div>
        </div>
    }
}

pub fn view_wide_data_row(label: Html, value: Html) -> Html {
    html! {
        <div class="row mb-1">
            <div class="col-lg-3 mb-1">{ label }</div>
            <div class="col-lg-9">{ value }</div>
        </div>
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Alert {
    Info,
    Warning,
    Danger,
}

impl Alert {
    /// `body` should have card formatting (e.g., `card-text` class on top-level `p` tags).
    pub fn view(self, title: &str, body: Html) -> Html {
        let icon = match self {
            Self::Info => Icon::Info,
            Self::Warning | Self::Danger => Icon::Warning,
        };
        let (text_class, border_class) = match self {
            Self::Info => ("text-info", "border-info"),
            Self::Warning => ("text-warning", "border-warning"),
            Self::Danger => ("text-danger", "border-danger"),
        };

        let top_img_classes = classes!["card-img-top", "fs-3", "text-center", text_class];

        html! {
            <div class=classes!["card", "card-alert", "my-4", border_class] role="alert">
                <div class=top_img_classes>
                    <span class="px-2 bg-white">{ icon.view() }</span>
                </div>
                <div class="card-body">
                    <h5 class=classes!["card-title", text_class]>{ title }</h5>
                    { body }
                </div>
            </div>
        }
    }
}

#[derive(Debug)]
pub struct ComponentRef<C: Component> {
    link: Rc<RefCell<Option<ComponentLink<C>>>>,
}

impl<C: Component> Default for ComponentRef<C> {
    fn default() -> Self {
        Self {
            link: Rc::new(RefCell::new(None)),
        }
    }
}

impl<C: Component> Clone for ComponentRef<C> {
    fn clone(&self) -> Self {
        Self {
            link: Rc::clone(&self.link),
        }
    }
}

impl<C: Component> PartialEq for ComponentRef<C> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.link, &other.link)
    }
}

impl<C: Component> ComponentRef<C> {
    pub fn link_with(self, link: ComponentLink<C>) {
        *self.link.borrow_mut() = Some(link);
    }

    pub fn send_message(&self, message: C::Message) {
        if let Some(link) = self.link.borrow().as_ref() {
            link.send_message(message);
        }
    }
}

#[derive(Debug)]
pub struct SavedStateManager {
    storage_key: &'static str,
    save: bool,
}

impl SavedStateManager {
    fn local_storage() -> web_sys::Storage {
        web_sys::window()
            .expect_throw("no window")
            .local_storage()
            .expect_throw("failed to get local_storage")
            .expect_throw("no local storage")
    }

    pub fn new(storage_key: &'static str, save: bool) -> (Self, Option<String>) {
        let saved = if save {
            Self::local_storage().get_item(storage_key).unwrap_or(None)
        } else {
            None
        };
        let this = Self { storage_key, save };
        (this, saved)
    }

    /// Returns `true` if the value was saved.
    pub fn save(&self, value: &str) -> bool {
        if self.save {
            Self::local_storage()
                .set_item(self.storage_key, value)
                .is_ok()
            // ^ the error here would not be fatal, so we just ignore it
        } else {
            false
        }
    }

    pub fn set_save_flag(&mut self, save: bool) {
        self.save = save;
        if !save {
            Self::local_storage().remove_item(self.storage_key).ok();
            // ^ the error here would not be fatal, so we just ignore it
        }
    }
}
