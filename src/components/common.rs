//! Common reused components.

use yew::{classes, html, Html};

use crate::fields::FieldWithValue;

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Link,
    Warning,
}

impl Icon {
    fn icon_class(self) -> &'static str {
        match self {
            Self::Link => "bi-link-45deg",
            Self::Warning => "bi-exclamation-diamond",
        }
    }

    pub fn to_html(self) -> Html {
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
                    rel="nofollow"
                    class="text-decoration-none"
                    title="View field definition">
                    { Icon::Link.to_html() }
                </a>
            </>
        }
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
