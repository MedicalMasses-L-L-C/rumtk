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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, SECTION_TITLES};
use crate::utils::types::{HTMLResult, SharedAppState, URLParams, URLPath};
use crate::utils::TextMap;
use crate::{
    rumtk_web_get_config, rumtk_web_get_config_string, rumtk_web_get_text_item, rumtk_web_render_template,
    RUMWebTemplate,
};

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/title.css' rel='stylesheet'>
        {% endif %}
        <div class='centered title-{{ css_class }}-container'>
            <h1 id='{{typ}}' class='title-{{ css_class }}'>{{ text }}</h1>
        </div>
    ",
    ext = "html"
)]
pub struct Title<'a> {
    typ: &'a str,
    text: &'a str,
    css_class: &'a str,
    custom_css_enabled: bool,
}

pub fn title(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    let text_store = rumtk_web_get_config_string!(state, SECTION_TITLES);
    let itm = rumtk_web_get_text_item!(&text_store, typ, &TextMap::default());
    let text = rumtk_web_get_text_item!(&itm, "title", typ);

    rumtk_web_render_template!(Title {
        typ,
        text,
        css_class,
        custom_css_enabled
    })
}
