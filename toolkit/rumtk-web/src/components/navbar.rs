use crate::components::logo::logo;
use crate::components::navlink::navlink;
use crate::components::COMPONENTS;
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS};
use crate::utils::types::{AppState, HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;
use axum::response::Html;
use phf_macros::phf_ordered_map;
use std::collections::HashMap;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>
            .navbar-default-container {
                position: fixed;
                top: 0;

                padding: 1em;

                display: flex;
                flex-direction: row;
                flex-wrap: wrap;

                align-items: center;
                justify-content: space-between;
                justify-items: center;

                background-color: var(--color-indigo);
                border-bottom: var(--color-turqoise) 0.1em solid;

                width: 100%;
                backdrop-filter: blur(5px);

                opacity: 0.9;
                height: fit-content;

                z-index: var(--top-layer);
            }

            .navbar-default-navlogo {
                position: relative;
                left: 0;
                min-width: 64px;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
            }

            .navbar-default-navactions {
                position: relative;

                align-self: center;
                width: fit-content;
                min-width: 200px;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
                justify-items: center;
                gap: 1em;

                padding: 10px;
            }

            .navbar-default-misc {
                position: relative;
                right: 0;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
                width: 17.5%;
                gap: 1em;
            }

            .navlink:link, .navlink:visited {
                color: var(--color-navlink);
            }

            .navlink:hover {
                background-color: var(--color-darkpurple);
                border-radius: 10px;
            }

            .brand-name {
                background-image: linear-gradient(to right, var(--color-darkpurple), var(--color-ticklemepink), var(--color-cerulean), var(--color-turqoise));
                background-clip: text;
                color: transparent;
            }

        </style>
        {% if custom_css_enabled %}
            <link href="/static/components/navbar.css" rel="stylesheet">
        {% endif %}
        <div class="navbar-{{ css_class }}-container">
            <div class="navbar-{{ css_class }}-navlogo">
                <a class="undecorated no-select" href="./" style="display:flex;flex-direction:row;align-items:center;">
                    {{logo|safe}}
                    <h3 class="brand-name"> MedicalMasses</h3>
                </a>
            </div>
            <div class="navbar-{{ css_class }}-navactions">
                {% for item in nav_links %}
                {{item|safe}}
                {% endfor %}
            </div>
            <div class="navbar-{{ css_class }}-misc">
            </div>
        </div>
    ",
    ext = "html"
)]
pub struct NavBar {
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
                app_state.clone(),
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
