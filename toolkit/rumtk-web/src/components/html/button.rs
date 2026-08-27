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
use crate::components::title::{title, Title};
use crate::utils::types::SharedAppState;
use crate::{rumtk_web_params_map, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe, PARAMS_TITLE};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <button onclick='{{ function }}()'>
            {{title | safe}}
        </button>
    ",
    ext = "html"
)]
pub struct Button {
    title: Title,
    function: RUMString,
}

impl RUMWebTemplateSafe for Button {}

pub fn button<'a>(text: &str, function: &str, state: SharedAppState) -> ComponentResult<Button> {
    let params = rumtk_web_params_map!([(PARAMS_TITLE, text)]);
    let title = title(&[], params.get_inner(), state)?;

    Ok(Button {
        title,
        function: function.to_string()
    })
}
