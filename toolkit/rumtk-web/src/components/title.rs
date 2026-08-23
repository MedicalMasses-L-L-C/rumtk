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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS};
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, rumtk_web_get_config_string, rumtk_web_get_text_item, ComponentResult, RUMWebTemplate, DEFAULT_TEXTMAP, PARAMS_ID, PARAMS_TITLE, SECTION_TITLES};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if !title.is_empty() %}
            {% if custom_css_enabled %}
                <link href='/static/components/title.css' rel='stylesheet'>
            {% endif %}
            <div class='centered title-{{ css_class }}-container'>
                <h1 id='{{id}}' class='title-{{ css_class }}'>{{ title }}</h1>
            </div>
        {% endif %}
    ",
    ext = "html"
)]
pub struct Title {
    id: RUMString,
    title: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn title<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<Title> {
    let id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_TEXT_ITEM).to_string();
    let text = rumtk_web_get_text_item!(params, PARAMS_TITLE, DEFAULT_TEXT_ITEM).to_string();
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();

    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    // Auto translation if config has title in another language
    let text_store = rumtk_web_get_config_string!(state, SECTION_TITLES);
    let itm = rumtk_web_get_text_item!(&text_store, &text, &DEFAULT_TEXTMAP);
    let title = rumtk_web_get_text_item!(&itm, "title", &text).to_string();

    Ok(Title {
        id,
        title,
        css_class,
        custom_css_enabled
    })
}
