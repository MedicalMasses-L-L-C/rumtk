use askama::Template;
use crate::{mm_render_html, mm_get_text_item, mm_get_misc_conf};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, TextMap, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, SECTION_SERVICES};

#[derive(Template, Debug, Clone)]
#[template(path = "components/item_card.html")]
struct ItemCard {
    services: &'static TextMap,
    css_class: MMString,
}

pub fn item_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, SECTION_SERVICES);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let services = mm_get_misc_conf!(typ);

    mm_render_html!(
        ItemCard {
            services,
            css_class: MMString::from(css_class),
        }
    )
}
