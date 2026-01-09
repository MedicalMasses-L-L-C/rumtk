use askama::Template;
use phf_macros::phf_ordered_map;
use crate::{mm_render_html, mm_get_conf, mm_get_text_item};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};

#[derive(Template, Debug)]
#[template(path = "components/title.html")]
struct Title {
    typ: MMString,
    text: MMString,
    css_class: MMString,
}

pub fn title(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let text_store = mm_get_conf!(SECTION_TITLES, DEFAULT_NO_TEXT);
    let en_text = mm_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_text, &typ, &&phf_ordered_map!());
    let text = MMString::from(mm_get_text_item!(&itm, "title", ""));

    mm_render_html!(
        Title {
            typ: MMString::from(typ),
            text,
            css_class: MMString::from(css_class),
        }
    )
}
