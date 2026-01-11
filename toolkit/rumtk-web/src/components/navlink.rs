use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TARGET};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_text_item, mm_render_html};
use askama::Template;
use phf_macros::phf_ordered_map;

#[derive(Debug, Clone)]
struct NavItem {
    title: &'static str,
    url: &'static str,
}

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/navlink.css' rel='stylesheet'>
        {% endif %}
        <a class='undecorated navlink f18' href='{{target.url}}'>{{target.title}}</a>
    ",
    ext = "html"
)]
pub struct NavLink {
    target: NavItem,
    css_class: MMString,
}

pub fn navlink(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let target = mm_get_text_item!(params, PARAMS_TARGET, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let links_store = mm_get_conf!(SECTION_LINKS, DEFAULT_NO_TEXT);
    let en_link = mm_get_text_item!(&links_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_link, &target, &&phf_ordered_map!());
    let title = mm_get_text_item!(&itm, "title", DEFAULT_NO_TEXT);
    let url = mm_get_text_item!(&itm, "url", DEFAULT_NO_TEXT);

    mm_render_html!(NavLink {
        target: NavItem { title, url },
        css_class: MMString::from(css_class),
    })
}
