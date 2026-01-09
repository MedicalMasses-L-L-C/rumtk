use askama::Template;
use crate::{mm_render_html, mm_get_text_item};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CONTENTS, PARAMS_CSS_CLASS};

#[derive(Template, Debug)]
#[template(path = "components/div.html")]
struct Div {
    text: MMString,
    css_class: MMString,
}

pub fn div(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let contents = mm_get_text_item!(params, PARAMS_CONTENTS, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    mm_render_html!(
        Div {
            text: MMString::from(contents),
            css_class: MMString::from(css_class),
        }
    )
}
