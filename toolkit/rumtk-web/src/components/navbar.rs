/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */
use crate::components::navlink::navlink;
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, SECTION_DEFAULT, SECTION_LINKS};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::utils::DEFAULT_NESTEDTEXTMAP;
use crate::{
    rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_component,
    rumtk_web_render_html,
};
use askama::Template;
use axum::response::Html;
use rumtk_core::strings::RUMStringConversions;
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

                padding: 10px;
            }

            .navbar-default-misc {
                position: relative;
                right: 0;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
                width: 17.5%;
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
            <link href='/static/components/navbar.css' rel='stylesheet'>
        {% endif %}
        <div class='navbar-{{ css_class }}-container'>
            <div class='navbar-{{ css_class }}-navlogo'>
                <a class='undecorated no-select' href='./' style='display:flex;flex-direction:row;align-items:center;'>
                    {{logo|safe}}
                    <h3 class='brand-name'> {{company}}</h3>
                </a>
            </div>
            <div class='navbar-{{ css_class }}-navactions gap0.10'>
                {% for item in nav_links %}
                {{item|safe}}
                {% endfor %}
            </div>
            <div class='navbar-{{ css_class }}-misc gap0.10'>
            </div>
        </div>
    ",
    ext = "html"
)]
pub struct NavBar {
    company: RUMString,
    logo: RUMString,
    nav_links: Vec<RUMString>,
    css_class: RUMString,
    custom_css_enabled: bool,
}

fn get_nav_links(keys: &Vec<&RUMString>, app_state: SharedAppConf) -> Vec<RUMString> {
    let mut nav_links = Vec::<RUMString>::with_capacity(keys.len());
    let default_html = Html::<String>(String::default());
    for key in keys {
        nav_links.push(
            navlink(
                &[],
                &HashMap::from([("target".to_rumstring(), key.to_rumstring())]),
                app_state.clone(),
            )
            .unwrap_or_else(|_| default_html.clone())
            .0
            .to_rumstring(),
        );
    }

    nav_links
}

pub fn navbar(_path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let company = state.lock().expect("Lock failure").title.clone();
    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let links_store = rumtk_web_get_string!(state, SECTION_LINKS);
    let en_link = rumtk_web_get_text_item!(&links_store, SECTION_DEFAULT, &DEFAULT_NESTEDTEXTMAP());
    let nav_keys = en_link.keys().collect::<Vec<&RUMString>>();
    let nav_links = get_nav_links(&nav_keys, state.clone());

    let logo =
        rumtk_web_render_component!("logo", [("type", "diamond"), ("class", "small")], state);

    rumtk_web_render_html!(NavBar {
        company: RUMString::from(company),
        logo,
        nav_links,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
