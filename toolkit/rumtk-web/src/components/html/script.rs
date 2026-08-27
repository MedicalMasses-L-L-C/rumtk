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
use crate::defaults::{DEFAULT_SCRIPT, DEFAULT_SCRIPT_MODULE,
};
use crate::js::rumtk_web_js_get_item;
use crate::utils::types::RUMString;
use crate::{ComponentResult, RUMWebTemplate};

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if typ.is_empty() || typ == DEFAULT_SCRIPT %}
            <script>{{script|safe}}</script>
        {% else if typ == DEFAULT_SCRIPT_MODULE %}
            <script type='module'>{{script|safe}}</script>
        {% else %}
            <script type='module' src='{{script}}' defer></script>
        {% endif %}
    ",
    ext = "html"
)]
pub struct Script {
    typ: RUMString,
    script: RUMString,
}

pub fn script<'a>(typ: &str, contents: &str, global: bool) -> ComponentResult<Script> {
    if !typ.is_empty() || typ != DEFAULT_SCRIPT_MODULE {
        Ok(Script {
            typ: typ.to_string(),
            script: match global {
                true => rumtk_web_js_get_item(contents),
                false => Ok(contents.to_string()),
            }?,
        })
    } else {
        Ok(Script {
            typ: typ.to_string(),
            script: contents.to_string(),
        })
    }
}
