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
use crate::utils::types::{HTMLResult, MMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_text_item, mm_render_html};
use askama::Template;
use phf_macros::phf_ordered_map;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>
            .title-default-container {
                display: block;
                height: 40px;
                align-content: center;
                margin-block: 20px 20px;
            }

            .title-default {
                display: block;
                margin-block: 0;
            }

            .title-default-overlay {
                position: relative;
                margin-block: 0;
                z-index: var(--mid-layer);
                bottom: 1.25em;

                background-image: var(--img-glitch-0);
                background-repeat: repeat;
                background-clip: text;
                color: transparent;
                background-position: center;
                filter: blur(5px);

                animation: slide 30s infinite linear;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/title.css' rel='stylesheet'>
        {% endif %}
        <div class='f14 centered title-{{ css_class }}-container'>
            <a id='{{typ}}'>
                <h2 class='title-{{ css_class }}'>{{ text.to_uppercase() }}</h2>
                <h2 class='title-{{ css_class }}-overlay no-select'>{{ text.to_uppercase() }}</h2>
            </a>
        </div>
    ",
    ext = "html"
)]
pub struct Title {
    typ: MMString,
    text: MMString,
    css_class: MMString,
    custom_css_enabled: bool,
}

pub fn title(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let text_store = mm_get_conf!(SECTION_TITLES, DEFAULT_NO_TEXT);
    let en_text = mm_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_text, &typ, &&phf_ordered_map!());
    let text = MMString::from(mm_get_text_item!(&itm, "title", ""));

    mm_render_html!(Title {
        typ: MMString::from(typ),
        text,
        css_class: MMString::from(css_class),
        custom_css_enabled
    })
}
