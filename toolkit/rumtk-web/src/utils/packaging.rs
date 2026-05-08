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

use minifier::{css, html, js, json};
use rumtk_core::core::RUMResult;
use rumtk_core::strings::RUMString;

pub enum Asset<'a> {
    CSS(&'a str),
    HTML(&'a str),
    JSON(&'a str),
    JS(&'a str),
}

pub fn minify_asset(asset: Asset) -> RUMResult<RUMString> {
    match asset {
        Asset::CSS(css) => match css::minify(css) {
            Ok(css) => Ok(css.to_string()),
            Err(err) => Err(err.to_string()),
        },
        Asset::HTML(html) => Ok(html::minify(html).to_string()),
        Asset::JSON(json) => Ok(json::minify(json).to_string()),
        Asset::JS(js) => Ok(js::minify(js).to_string()),
    }
}
