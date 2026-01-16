use crate::components::{app_body::app_body, app_head::app_head};
/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, LANG_EN};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_html};
use askama::Template;

#[derive(Template)]
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

pub fn app_shell(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let lang = rumtk_web_get_text_item!(params, "lang", LANG_EN);
    let theme = rumtk_web_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);
    // TODO: We need to reevaluate how to validate the options that should be standardized to avoid parameter injection as an attack vector.
    //owned_state.opts = *params.clone();

    //Config App
    state.write().expect("Lock failure").lang = RUMString::from(lang);
    state.write().expect("Lock failure").theme = RUMString::from(theme);

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
