/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS};
use crate::{rumtk_web_get_text_item, ComponentResult};
use crate::{RUMWebTemplate, SharedAppState, URLParams, URLPath};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        <div class='loader-{{css_class}}'></div>
    ",
    ext = "html"
)]
pub struct Loader {
    css_class: RUMString,
}

pub fn loader<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<Loader> {
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();


    Ok(Loader {
            css_class
        }
    )
}
