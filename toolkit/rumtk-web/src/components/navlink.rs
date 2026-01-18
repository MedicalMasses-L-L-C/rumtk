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
use crate::utils::defaults::{
    DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TARGET, SECTION_LINKS,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::utils::{DEFAULT_TEXT, DEFAULT_TEXTMAP};
use crate::{rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;

#[derive(Debug, Clone)]
struct NavItem<'a> {
    title: &'a str,
    url: &'a str,
}

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>
            .navlink:link, .navlink:visited {
                color: var(--color-navlink);
            }

            .navlink:hover {
                background-color: var(--color-darkpurple);
                border-radius: 10px;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/navlink.css' rel='stylesheet'>
        {% endif %}
        <a class='undecorated navlink f18 navlink-{{css_class}}' href='{{target.url}}'>{{target.title}}</a>
    ",
    ext = "html"
)]
pub struct NavLink<'a> {
    target: NavItem<'a>,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn navlink(_path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let target = rumtk_web_get_text_item!(params, PARAMS_TARGET, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.read().expect("Lock failure").custom_css;

    let links_store = rumtk_web_get_string!(state, SECTION_LINKS);
    let itm = rumtk_web_get_text_item!(&links_store, target, &DEFAULT_TEXTMAP());
    let title = rumtk_web_get_text_item!(&itm, "title", &DEFAULT_TEXT());
    let url = rumtk_web_get_text_item!(&itm, "url", &DEFAULT_TEXT());

    rumtk_web_render_html!(NavLink {
        target: NavItem { title, url },
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
