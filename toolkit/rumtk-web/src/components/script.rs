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
use crate::defaults::{
    DEFAULT_NO_TEXT, DEFAULT_SCRIPT, DEFAULT_SCRIPT_MODULE, PARAMS_ID, PARAMS_TYPE,
};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CONTENTS};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_text_item, rumtk_web_render_html, RUMWebTemplate};
use askama::Template;

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        {% if typ.is_empty() || typ == DEFAULT_SCRIPT %}
            <script id={{id}}>{{script|safe}}</script>
        {% else if typ == DEFAULT_SCRIPT_MODULE %}
            <script type='module' id={{id}}>{{script|safe}}</script>
        {% else %}
            <script type='module' src='{{script|safe}}' id={{id}}></script>
        {% endif %}
    ",
    ext = "html"
)]
pub struct Script<'a> {
    id: &'a str,
    typ: &'a str,
    script: RUMString,
}

pub fn script(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_NO_TEXT);
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_SCRIPT);
    let contents = rumtk_web_get_text_item!(params, PARAMS_CONTENTS, DEFAULT_TEXT_ITEM);

    rumtk_web_render_html!(Script {
        id,
        typ,
        script: RUMString::from(contents),
    })
}
