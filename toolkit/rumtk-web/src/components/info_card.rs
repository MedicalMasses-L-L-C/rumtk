use askama::Template;
use phf_macros::{phf_ordered_map};
use crate::{mm_get_param_eq, mm_render_html, mm_get_conf, mm_get_text_item};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, OPT_INVERTED_DIRECTION, DEFAULT_NO_TEXT, PARAMS_CSS_CLASS, PARAMS_INVERTED, PARAMS_ITEM};

#[derive(Template, Debug, Clone)]
#[template(path = "components/info_card.html")]
struct InfoCard {
    title: &'static str,
    description: &'static str,
    inverted: bool,
    css_class: MMString,
}

pub fn info_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let card_text_item = mm_get_text_item!(params, PARAMS_ITEM, DEFAULT_TEXT_ITEM);
    let inverted = mm_get_param_eq!(params, PARAMS_INVERTED, OPT_INVERTED_DIRECTION, false);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let text_store = mm_get_conf!(SECTION_TEXT, DEFAULT_NO_TEXT);
    let en_text = mm_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_text, &card_text_item, &&phf_ordered_map!());
    let title = mm_get_text_item!(&itm, "title", DEFAULT_NO_TEXT);
    let desc = mm_get_text_item!(&itm, "description", DEFAULT_NO_TEXT);

    mm_render_html!(
        InfoCard {
            title,
            description: desc,
            inverted,
            css_class: MMString::from(css_class),
        }
    )
}
