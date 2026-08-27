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
use crate::components::html::{button, script, Button, Script};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_FUNCTION,
};
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, rumtk_web_get_text_item, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe, DEFAULT_GOTO_FUNCTION, DEFAULT_SCRIPT_IMPORT, PARAMS_TITLE};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/contact_button.css' rel='stylesheet'>
        {% endif %}
        {{script|safe}}
        <div id={{id}} class='contact-{{ css_class }}-button-container'>
            {{button|safe}}
        </div>
    ",
    ext = "html"
)]
pub struct ContactButton {
    id: RUMString,
    button: Button,
    script: Script,
    css_class: RUMString,
    custom_css_enabled: bool,
}

impl RUMWebTemplateSafe for ContactButton {}

pub fn contact_button<'a>(
    _path_components: URLPath<'a, 'a>,
    params: URLParams<'a>,
    state: SharedAppState,
) -> ComponentResult<ContactButton> {
    let title = rumtk_web_get_text_item!(params, PARAMS_TITLE, DEFAULT_TEXT_ITEM);
    let function = rumtk_web_get_text_item!(params, PARAMS_FUNCTION, DEFAULT_GOTO_FUNCTION);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();

    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    let button = button(title, function, "'./contact'", state.clone())?;
    let script = script(DEFAULT_SCRIPT_IMPORT, "goto", true)?;

    Ok(ContactButton {
        id: RUMString::from("contact_button"),
        button,
        script,
        css_class,
        custom_css_enabled
    })
}
