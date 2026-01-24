/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, SECTION_LINKS};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_component,
    rumtk_web_render_html, RUMWebData, RUMWebTemplate,
};
use askama::Template;
use axum::response::Html;
use rumtk_core::strings::RUMStringConversions;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <style>
            .brand-name {
                background-image: linear-gradient(to right, var(--color-darkpurple), var(--color-ticklemepink), var(--color-cerulean), var(--color-turqoise));
                background-clip: text;
                color: transparent;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/header.css' rel='stylesheet'>
        {% endif %}
        <header class='header-{{ css_class }}-container'>
            {% if !disable_logo %}
            <div class='header-{{ css_class }}-navlogo'>
                <a class='undecorated no-select' href='./' style='display:flex;flex-direction:row;align-items:center;'>
                    {{logo|safe}}
                    <h3 class='brand-name'> {{company}}</h3>
                </a>
            </div>
            {% endif %}
            <div class='header-{{ css_class }}-navactions gap-10'>
                {% for item in nav_links %}
                    {{item|safe}}
                {% endfor %}
            </div>
            <div class='header-{{ css_class }}-misc gap-10'>
            </div>
        </header>
    ",
    ext = "html"
)]
pub struct Header {
    company: RUMString,
    logo: RUMString,
    nav_links: Vec<RUMString>,
    css_class: RUMString,
    custom_css_enabled: bool,
    disable_logo: bool,
}

fn get_nav_links(keys: &Vec<&RUMString>, app_state: SharedAppState) -> Vec<RUMString> {
    let mut nav_links = Vec::<RUMString>::with_capacity(keys.len());
    let default_html = Html::<String>(String::default());
    for key in keys {
        nav_links.push(
            navlink(
                &[],
                &RUMWebData::from([("target".to_rumstring(), key.to_rumstring())]),
                app_state.clone(),
            )
            .unwrap_or_default()
            .to_rumstring(),
        );
    }

    nav_links
}

pub fn header(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let company = state.read().expect("Lock failure").config.company.clone();
    let custom_css_enabled = state.read().expect("Lock failure").config.custom_css;

    let links_store = rumtk_web_get_string!(state, SECTION_LINKS);
    let nav_keys = links_store.keys().collect::<Vec<&RUMString>>();
    let nav_links = match state
        .read()
        .expect("Lock failure")
        .config
        .header_conf
        .disable_navlinks
    {
        true => vec![rumtk_web_render_component!(
            "title",
            [(
                "type",
                &state.read().expect("Lock failure").config.title.clone()
            )],
            state
        )],
        false => get_nav_links(&nav_keys, state.clone()),
    };

    let disable_logo = state
        .read()
        .expect("Lock failure")
        .config
        .header_conf
        .disable_logo;
    let logo = match disable_logo {
        true => RUMString::default(),
        false => rumtk_web_render_component!(
            "logo",
            [
                ("type", "diamond"),
                (
                    "class",
                    state
                        .read()
                        .expect("Lock failure")
                        .config
                        .header_conf
                        .logo_size
                        .as_str()
                ),
            ],
            state
        ),
    };

    rumtk_web_render_html!(Header {
        company: RUMString::from(company),
        logo,
        nav_links,
        css_class: RUMString::from(css_class),
        custom_css_enabled,
        disable_logo
    })
}
