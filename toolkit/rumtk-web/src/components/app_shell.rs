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
use crate::components::{app_body::app_body, app_head::app_head};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, LANG_EN};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_html, rumtk_web_set_config,
    RUMWebTemplate,
};
use rumtk_core::{rumtk_critical_section_read, rumtk_critical_section_write};

#[derive(RUMWebTemplate)]
#[template(
    source = "
        <!DOCTYPE html>
        <html lang='{{lang}}'>
            {{head|safe}}
            {{body|safe}}
        </html>
    ",
    ext = "html"
)]
pub struct AppShell {
    head: RUMString,
    lang: RUMString,
    body: RUMString,
}

pub fn app_shell(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let lang = rumtk_web_get_text_item!(params, "lang", LANG_EN);
    let theme = rumtk_web_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);
    // TODO: We need to reevaluate how to validate the options that should be standardized to avoid parameter injection as an attack vector.
    //owned_state.opts = *params.clone();

    //Config App
    rumtk_web_set_config!(state).lang = RUMString::from(lang);
    rumtk_web_set_config!(state).theme = RUMString::from(theme);

    //Let's render the head component
    let head = rumtk_web_render_component!(|| -> HTMLResult {
        app_head(path_components, params, state.clone())
    });

    //Let's render the head component
    let body = rumtk_web_render_component!(|| -> HTMLResult {
        app_body(path_components, params, state.clone())
    });

    rumtk_web_render_html!(AppShell {
        lang: RUMString::from(lang),
        head,
        body
    })
}
