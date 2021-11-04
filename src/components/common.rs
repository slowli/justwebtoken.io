//! Common reused components.

use yew::{classes, html, Html};

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
                <>
                    <label class="ms-md-3 text-decoration-underline">{ field.name }</label>
                    { if let Some(link) = field.link {
                        Self::view_link(link)
                    } else {
                        html!{}
                    }}
                </>
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
                { " " }
                <a href=link
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
            <div class="col-md-3">{ label }</div>
            <div class="col-md-9">{ value }</div>
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
