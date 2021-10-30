//! Common reused components.

use yew::{html, Html};

use crate::fields::FieldWithValue;

impl FieldWithValue<'_> {
    // FIXME: render link
    pub fn view_aux(&self) -> Html {
        let field = &self.field;
        view_data_row(
            html! { <label class="ms-md-3 text-decoration-underline">{ field.name }</label> },
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
}

fn str_to_html(html_str: &str) -> Html {
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
