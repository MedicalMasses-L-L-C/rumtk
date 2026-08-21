/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::defaults::PARAMS_ID;
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS};
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, rumtk_web_get_text_item, ComponentResult, RUMWebTemplate};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/anchor.css' rel='preload' as='style' >
            <link href='/static/components/a.css' rel='stylesheet'>
        {% endif %}
        <a class='anchor-{{css_class}}' href='#{{id}}'></div>
    ",
    ext = "html"
)]
pub struct Anchor {
    id: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn anchor<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<Anchor> {
    let id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_TEXT_ITEM).to_string();
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();

    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    Ok(Anchor {
        id,
        css_class,
        custom_css_enabled
    })
}

pub fn a<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<Anchor> {
    anchor(_path_components, params, state)
}
