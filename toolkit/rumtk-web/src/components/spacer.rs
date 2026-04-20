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
use crate::utils::defaults::{PARAMS_SIZE, SECTION_DEFAULT};
use crate::utils::types::{HTMLResult, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_config, rumtk_web_get_text_item, rumtk_web_render_html, AppConf, RUMWebTemplate,
};
use askama::Template;

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/spacer.css' rel='stylesheet'>
        {% endif %}
        <div style='padding-bottom: {{size}}0px'></div>
    ",
    ext = "html"
)]
pub struct Spacer {
    size: usize,
    custom_css_enabled: bool,
}

pub fn spacer(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let size = rumtk_web_get_text_item!(params, PARAMS_SIZE, SECTION_DEFAULT)
        .parse::<usize>()
        .unwrap_or(0);

    let custom_css_enabled = rumtk_web_get_config!(state).custom_css;

    rumtk_web_render_html!(Spacer {
        size,
        custom_css_enabled
    })
}
