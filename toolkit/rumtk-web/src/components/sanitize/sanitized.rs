/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
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
use crate::{sanitize_html, ComponentResult, RUMWebTemplate};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {{contents|safe}}
    ",
    ext = "html"
)]
pub struct Sanitized {
    contents: RUMString,
}

#[inline]
pub fn sanitized<T: RUMWebTemplate>(contents: T) -> ComponentResult<Sanitized> {
    let inner = contents.to_string();
    let contents = sanitize_html(&inner);

    Ok(Sanitized {
        contents,
    })
}
