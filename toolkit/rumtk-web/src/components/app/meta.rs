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
use crate::utils::types::SharedAppState;
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate)]
#[template(
    source = "
            <meta charset='UTF-8'>
            <meta http-equiv='Content-Type' content='text/html; charset=utf-8' />
            <meta name='viewport' content='width=device-width, initial-scale=1 shrink-to-fit=no' />
            <meta http-equiv='X-UA-Compatible' content='IE=edge,chrome=1'/>
            <meta name='description' content='{{description}}'>
            <title>{{title}}</title>
    ",
    ext = "html"
)]
pub struct Meta {
    title: RUMString,
    description: RUMString,
}

impl RUMWebTemplateSafe for Meta {}

#[inline]
pub fn meta(state: SharedAppState) -> ComponentResult<Meta> {
    Ok(Meta {
        title: rumtk_web_get_config!(state).title.to_string(),
        description: rumtk_web_get_config!(state).description.to_string()
    })
}
