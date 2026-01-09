use askama::Template;
use phf_macros::phf_ordered_map;
use crate::{mm_get_param, mm_render_html, mm_get_conf, mm_get_text_item};
use crate::utils::types::{HTMLResult, MMString, NestedNestedTextMap, NestedTextMap, SharedAppState, TextMap, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, DEFAULT_NO_TEXT, DEFAULT_CONTACT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, PARAMS_SECTION};

#[derive(Template, Debug)]
#[template(path = "components/contact_card.html")]
struct ContactCard {
    contact_lines: &'static TextMap,
    css_class: MMString,
}

pub fn contact_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let section = mm_get_text_item!(params, PARAMS_SECTION, DEFAULT_CONTACT_ITEM);
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_CONTACT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let text_conf: &NestedNestedTextMap = mm_get_conf!(SECTION_CONTACT, DEFAULT_NO_TEXT);
    let contact_item: &&NestedTextMap = mm_get_text_item!(&text_conf, &section, &&phf_ordered_map!());
    let contact_lines: &TextMap = mm_get_text_item!(&contact_item, &typ, &phf_ordered_map!());

    mm_render_html!(
        ContactCard {
            contact_lines,
            css_class: MMString::from(css_class),
        }
    )
}
