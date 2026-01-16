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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{rumtk_web_get_param_eq, rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/logo.css' rel='stylesheet'>
        {% endif %}
        <div class='centered logo'>
        {% if diamond %}
            <img src='/static/img/logo.webp' alt='Webp Logo' class='logo-{{ css_class }}' fetchpriority='high' />
        {% else %}
            <img src='/static/img/logo.svg' alt='SVG Logo' fetchpriority='high'/>
        {% endif %}
        </div>
    ",
    ext = "html"
)]
pub struct Logo {
    diamond: bool,
    css_class: RUMString,
    custom_css_enabled: bool,
}

const DEFAULT_TYPE: &str = "diamond";

pub fn logo(_path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let diamond = rumtk_web_get_param_eq!(params, PARAMS_TYPE, DEFAULT_TYPE, false);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.read().expect("Lock failure").custom_css;

    rumtk_web_render_html!(Logo {
        diamond,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
