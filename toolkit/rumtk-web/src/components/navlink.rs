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
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TARGET};
use crate::utils::types::{HTMLResult, MMString, SharedAppConf, URLParams, URLPath};
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
    custom_css_enabled: bool,
}

pub fn navlink(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let target = mm_get_text_item!(params, PARAMS_TARGET, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let links_store = mm_get_conf!(SECTION_LINKS, DEFAULT_NO_TEXT);
    let en_link = mm_get_text_item!(&links_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_link, &target, &&phf_ordered_map!());
    let title = mm_get_text_item!(&itm, "title", DEFAULT_NO_TEXT);
    let url = mm_get_text_item!(&itm, "url", DEFAULT_NO_TEXT);

    mm_render_html!(NavLink {
        target: NavItem { title, url },
        css_class: MMString::from(css_class),
        custom_css_enabled
    })
}
