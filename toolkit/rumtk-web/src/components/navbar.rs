use std::collections::HashMap;
use askama::Template;
use axum::response::Html;
use phf_macros::phf_ordered_map;
use crate::{mm_get_conf, mm_get_text_item, mm_render_component, mm_render_html};
use crate::components::COMPONENTS;
use crate::components::logo::logo;
use crate::components::navlink::navlink;
use crate::utils::types::{AppState, HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS};

#[derive(Template, Debug, Clone)]
#[template(path = "components/navbar.html")]
struct NavBar {
    logo: MMString,
    nav_links: Vec<MMString>,
    css_class: MMString,
}

fn get_nav_links(keys: &Vec<&&str>, app_state: SharedAppState) -> Vec<MMString> {
    let mut nav_links = Vec::with_capacity(keys.len());
    let default_html = Html::<MMString>(MMString::default());
    for key in keys {
        nav_links.push(
            navlink(
                &[],
                &HashMap::from([
                    ("target".to_string(), key.to_string()),
                ]),
                app_state.clone()
            ).unwrap_or_else(|_| default_html.clone()).0
        );
    }

    nav_links
}

pub fn navbar(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let links_store = mm_get_conf!(SECTION_LINKS, DEFAULT_NO_TEXT);
    let en_link = mm_get_text_item!(&links_store, "0", &&phf_ordered_map!());
    let nav_keys = en_link.keys().collect::<Vec<&&str>>();
    let nav_links = get_nav_links(&nav_keys, state.clone());

    let logo = mm_render_component!("logo", [("type", "diamond"), ("class", "small")], state, COMPONENTS);

    mm_render_html!(
        NavBar {
            logo,
            nav_links,
            css_class: MMString::from(css_class),
        }
    )
}
