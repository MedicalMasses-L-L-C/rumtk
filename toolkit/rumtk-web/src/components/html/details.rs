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
use crate::components::html::pre::{pre, Pre};
use crate::components::html::summary::{summary, Summary};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS};
use crate::utils::types::{HTMLResult, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_config, rumtk_web_get_text_item,
    rumtk_web_render_template, RUMWebTemplate,
};

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/details.css' rel='stylesheet'>
        {% endif %}
        <details class='details-{{css_class}}'>
            {{summary}}
            {{contents}}
        </details>
    ",
    ext = "html"
)]
pub struct Details<'a> {
    summary: Summary<'a>,
    contents: Pre<'a>,
    css_class: &'a str,
    custom_css_enabled: bool,
}

pub fn details(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    rumtk_web_render_template!(Details {
        summary: summary(_path_components, params, state.clone())?,
        contents: pre(_path_components, params, state)?,
        css_class,
        custom_css_enabled
    })
}
