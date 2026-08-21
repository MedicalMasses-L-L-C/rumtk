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
use crate::components::contact_button::{contact_button, ContactButton};
use crate::components::html::{footer, Footer};
use crate::components::socials::{socials, Socials};
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <p class='f16'>
            {{company}} &copy; {{copyright}}
        </p>
        {% if !disable_contact_button %}
        {{button}}
        {% endif %}
        {{socials}}
    ",
    ext = "html"
)]
pub struct FooterContents {
    company: RUMString,
    copyright: RUMString,
    button: ContactButton,
    socials: Socials,
    disable_contact_button: bool
}

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {{footer}}
    ",
    ext = "html"
)]
pub struct AppFooter {
    footer: Footer<FooterContents>
}

pub fn app_footer<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<AppFooter> {
    let company = rumtk_web_get_config!(state).company.clone();
    let copyright = rumtk_web_get_config!(state).copyright.clone();

    let disable_contact_button = rumtk_web_get_config!(state).footer_conf.disable_contact_button;
    let contact_button = contact_button(
                _path_components,
                params,
                state.clone()
            )?;

    let socials = socials(_path_components, params, state.clone())?;

    let footer_contents = FooterContents {
        company,
        copyright,
        button: contact_button,
        socials,
        disable_contact_button
    };

    let app_footer = footer(
        footer_contents,
        params,
        state
    )?;

    Ok(AppFooter {
        footer: app_footer,
    })
}
