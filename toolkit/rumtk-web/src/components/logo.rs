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
///
/// Logo component module.
///
/// ## Accepts Parameters
/// * [PARAMS_SOURCE_URL] => URL from which to obtain the logo image. Defaults to `/static/img/logo.webp`.
/// * [PARAMS_CSS_CLASS] => Which variant of CSS styling to use. Defaults to `default` => `logo-default`.
///
use crate::utils::defaults::{DEFAULT_LOGO_SOURCE, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOURCE_URL};
use crate::utils::types::{HTMLResult, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_config, rumtk_web_get_text_item, rumtk_web_render_html,
    RUMWebTemplate,
};


#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/logo.css' rel='stylesheet'>
        {% endif %}
        <div class='centered logo'>
        <img src='{{ source }}' alt='Logo' class='logo-{{ css_class }}' fetchpriority='high' />
        </div>
    ",
    ext = "html"
)]
pub struct Logo<'a> {
    source: &'a str,
    css_class: &'a str,
    custom_css_enabled: bool,
}

pub fn logo(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let source = rumtk_web_get_text_item!(params, PARAMS_SOURCE_URL, DEFAULT_LOGO_SOURCE);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = rumtk_web_get_config!(state).custom_css;

    rumtk_web_render_html!(Logo {
        source,
        css_class,
        custom_css_enabled
    })
}
