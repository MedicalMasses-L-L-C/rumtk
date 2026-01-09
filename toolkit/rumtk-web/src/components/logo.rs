use askama::Template;
use crate::{mm_get_param_eq, mm_render_html, mm_get_text_item};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};

#[derive(Template, Debug, Clone)]
#[template(path = "components/logo.html")]
struct Logo {
    diamond: bool,
    css_class: MMString,
}

const DEFAULT_TYPE: &str = "diamond";

pub fn logo(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let diamond = mm_get_param_eq!(params, PARAMS_TYPE, DEFAULT_TYPE, false);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    mm_render_html!(
        Logo {
            diamond,
            css_class: MMString::from(css_class)
        }
    )
}
