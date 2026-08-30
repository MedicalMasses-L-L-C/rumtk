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

use crate::utils::types::SharedAppState;
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate, DEFAULT_ICON_SOURCE, DEFAULT_ICON_TYPE};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <link rel='icon' type='{{typ}}' href='{{source}}'>
    ",
    ext = "html"
)]
pub struct FavIcon {
    src: RUMString,
    typ: RUMString,
}

pub fn favicon<'a>(state: SharedAppState) -> ComponentResult<FavIcon> {
    let src = rumtk_web_get_config!(state).header_conf.icon_source.clone().unwrap_or(DEFAULT_ICON_SOURCE.to_string());
    let typ = rumtk_web_get_config!(state).header_conf.icon_type.clone().unwrap_or(DEFAULT_ICON_TYPE.to_string());

    Ok(FavIcon {
        src,
        typ,
    })
}
