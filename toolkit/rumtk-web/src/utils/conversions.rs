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
use rumtk_core::strings::{rumtk_format, string_to_b64, RUMString};

///
/// Turn a payload into a `base64` encoded, `data` url to embed payload into tag.
///
/// ## Example
/// ```
/// ```
///
pub fn to_data_uri(data: &str, mime: &str) -> RUMString {
    // data:image/svg+xml;base64,
    let b64 = string_to_b64(data);
    rumtk_format!("data:{mime};base64,{}", b64)
}
