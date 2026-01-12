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
use crate::utils::defaults::{
    DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, SECTION_DEFAULT, SECTION_TITLES,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::utils::{NestedTextMap, TextMap};
use crate::{rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;

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
    typ: RUMString,
    text: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn title(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let text_store = rumtk_web_get_string!(state, SECTION_TITLES);
    let en_text = rumtk_web_get_text_item!(&text_store, SECTION_DEFAULT, &NestedTextMap::default());
    let itm = rumtk_web_get_text_item!(&en_text, typ, &TextMap::default());
    let text = RUMString::from(rumtk_web_get_text_item!(&itm, "title", ""));

    rumtk_web_render_html!(Title {
        typ: RUMString::from(typ),
        text,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
