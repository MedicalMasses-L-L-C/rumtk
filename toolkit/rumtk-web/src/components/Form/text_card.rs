use askama::Template;
use crate::{mm_render_html, mm_get_text_item};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_ITEM, PARAMS_TYPE};

#[derive(Template, Debug, Clone)]
#[template(path = "components/form/text_card.html")]
struct TextCard {
    typ: MMString,
    css_class: MMString,
}

pub fn text_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    mm_render_html!(
        TextCard {
            typ: MMString::from(typ),
            css_class: MMString::from(css_class),
        }
    )
}
