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
use crate::utils::types::{SharedAppState, URLParams};
use crate::{rumtk_web_get_config, rumtk_web_get_text_item, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/header.css' rel='stylesheet'>
        {% endif %}
        <header class='header-{{ css_class }}-container header'>
            {{contents|safe}}
        </header>
    ",
    ext = "html"
)]
pub struct Header<T: RUMWebTemplate> {
    contents: T,
    css_class: RUMString,
    custom_css_enabled: bool,
}

impl<T: RUMWebTemplate + RUMWebTemplateSafe> RUMWebTemplateSafe for Header<T> {}

pub fn header<T: RUMWebTemplate + RUMWebTemplateSafe>(contents: T, params: URLParams, state: SharedAppState) -> ComponentResult<Header<T>> {
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();

    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    Ok(Header {
        contents,
        css_class,
        custom_css_enabled,
    })
}
