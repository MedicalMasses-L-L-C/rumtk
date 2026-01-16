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
use crate::utils::defaults::{PARAMS_SIZE, SECTION_DEFAULT};
use crate::utils::types::{HTMLResult, SharedAppConf, URLParams, URLPath};
use crate::{rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>

        </style>
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

pub fn spacer(_path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let size = rumtk_web_get_text_item!(params, PARAMS_SIZE, SECTION_DEFAULT)
        .parse::<usize>()
        .unwrap_or(0);

    let custom_css_enabled = state.read().expect("Lock failure").custom_css;

    rumtk_web_render_html!(Spacer {
        size,
        custom_css_enabled
    })
}
