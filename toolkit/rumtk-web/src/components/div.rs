use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CONTENTS, PARAMS_CSS_CLASS};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_html};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/div.css' rel='stylesheet'>
        {% endif %}
        <div class='div-{{css_class}}'>{{contents|safe}}</div>
    ",
    ext = "html"
)]
pub struct Div {
    contents: MMString,
    css_class: MMString,
    custom_css_enabled: bool,
}

pub fn div(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let contents = mm_get_text_item!(params, PARAMS_CONTENTS, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    mm_render_html!(Div {
        contents: MMString::from(contents),
        css_class: MMString::from(css_class),
        custom_css_enabled
    })
}
