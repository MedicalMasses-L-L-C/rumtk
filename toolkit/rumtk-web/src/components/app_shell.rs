use crate::components::{app_body::app_body, app_head::app_head};
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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, LANG_EN};
use crate::utils::types::{HTMLResult, MMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;

const DEFAULT_PAGE_NAME: &str = "index";

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
    head: MMString,
    lang: MMString,
    body: MMString,
}

pub fn app_shell(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let lang = mm_get_text_item!(params, "lang", LANG_EN);
    let theme = mm_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);
    // TODO: We need to reevaluate how to validate the options that should be standardized to avoid parameter injection as an attack vector.
    //owned_state.opts = *params.clone();

    //Config App
    let mut owned_state = state.lock().expect("Lock failure");
    owned_state.lang = MMString::from(lang);
    owned_state.theme = MMString::from(theme);

    //Let's render the head component
    let head =
        mm_render_component!(|| -> HTMLResult { app_head(path_components, params, state.clone()) });

    //Let's render the head component
    let body =
        mm_render_component!(|| -> HTMLResult { app_body(path_components, params, state.clone()) });

    mm_render_html!(AppShell {
        lang: MMString::from(lang),
        head,
        body
    })
}
