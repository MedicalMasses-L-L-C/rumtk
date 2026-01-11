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
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{
    rumtk_web_get_conf, rumtk_web_get_text_item, rumtk_web_render_html, rumtk_web_render_markdown,
};
use askama::Template;
use phf_macros::phf_ordered_map;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>
            .label-default {
                text-wrap: wrap;
                margin: auto;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/form/label.css' rel='stylesheet'>
        {% endif %}
        <pre class='label-{{css_class}}'>
            {{text|safe}}
        </pre>
    ",
    ext = "html"
)]
struct Label {
    text: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn label(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let text_store = rumtk_web_get_conf!(SECTION_TEXT, DEFAULT_NO_TEXT);
    let en_text = rumtk_web_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = rumtk_web_get_text_item!(&en_text, &typ, &&phf_ordered_map!());
    let desc = rumtk_web_get_text_item!(&itm, "description", DEFAULT_NO_TEXT);
    let html = rumtk_web_render_markdown!(desc);

    rumtk_web_render_html!(Label {
        text: html,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
