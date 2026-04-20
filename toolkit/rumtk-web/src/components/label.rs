/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::utils::defaults::{
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, SECTION_TEXT,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::utils::DEFAULT_TEXTMAP;
use crate::{
    rumtk_web_get_config, rumtk_web_get_config_string, rumtk_web_get_text_item, rumtk_web_render_html,
    rumtk_web_render_markdown, RUMWebTemplate,
};

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/label.css' rel='stylesheet'>
        {% endif %}
        <pre class='label-{{css_class}}'>
            {{text|safe}}
        </pre>
    ",
    ext = "html"
)]
pub struct Label {
    text: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn label(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = rumtk_web_get_config!(state).custom_css;

    let text_store = rumtk_web_get_config_string!(state, SECTION_TEXT);
    let itm = rumtk_web_get_text_item!(&text_store, typ, &DEFAULT_TEXTMAP());
    let desc = rumtk_web_get_text_item!(&itm, "description", DEFAULT_NO_TEXT);
    let html = rumtk_web_render_markdown!(desc);

    rumtk_web_render_html!(Label {
        text: html,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
