/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
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

use crate::components::form::props::InputProps;
use crate::components::form::select_element::select_element;
use crate::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, ELEMENT_SELECT, PARAMS_CONTENTS, PARAMS_CSS_CLASS};
use crate::utils::types::{HTMLResult, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_config, rumtk_web_get_text_item, rumtk_web_render_template, RUMWebTemplate,
};

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/div.css' rel='stylesheet'>
        {% endif %}
        <div class='div-{{css_class}}'>
        {{contents|safe}}
        </div>
    ",
    ext = "html"
)]
pub struct Select<'a> {
    contents: &'a str,
    css_class: &'a str,
    custom_css_enabled: bool,
}

pub fn select(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let items = rumtk_web_get_text_item!(params, PARAMS_CONTENTS, DEFAULT_NO_TEXT);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = rumtk_web_get_config!(state).custom_css;

    let props = InputProps::default();
    let contents = &select_element(ELEMENT_SELECT, items, props, DEFAULT_NO_TEXT)?.to_string();

    rumtk_web_render_template!(Select {
        contents,
        css_class,
        custom_css_enabled
    })
}