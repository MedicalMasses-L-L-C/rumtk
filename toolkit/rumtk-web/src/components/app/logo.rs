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
use crate::components::html::{img, Img};
///
/// Logo component module.
///
/// ## Accepts Parameters
/// * [PARAMS_SOURCE_URL] => URL from which to obtain the logo image. Defaults to `/static/img/logo.webp`.
/// * [PARAMS_CSS_CLASS] => Which variant of CSS styling to use. Defaults to `default` => `logo-default`.
///
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOURCE_URL};
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, rumtk_web_get_text_item, rumtk_web_params_map, ComponentResult, RUMWebTemplate, DEFAULT_LOGO_SOURCE};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/logo.css' rel='stylesheet'>
        {% endif %}
        <div class='centered logo'>{{img | safe}}</div>
    ",
    ext = "html"
)]
pub struct Logo {
    img: Img,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn logo<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<Logo> {
    let source = rumtk_web_get_config!(state).header_conf.logo_source.clone().unwrap_or(DEFAULT_LOGO_SOURCE.to_string());
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();

    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    let params = rumtk_web_params_map!([(PARAMS_CSS_CLASS, &css_class)]);

    Ok(Logo {
        img: img(source, params.get_inner(), state)?,
        css_class,
        custom_css_enabled
    })
}
