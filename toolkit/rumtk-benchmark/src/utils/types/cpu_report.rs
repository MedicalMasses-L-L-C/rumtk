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
use rumtk_core::buffers::buffer_to_string;
use rumtk_core::search::rumtk_search::string_find_value;
use rumtk_core::serde::{RUMDeJson, RUMSerJson};
use rumtk_core::strings::RUMString;
use rumtk_core::types::RUMBuffer;
use rumtk_web::conversions::to_data_uri;
use rumtk_web::RUMWebTemplate;

///
/// Extracts basic call stack information for later display. Note, this type should be paired with
/// the output of `flamegraph` (See the crate [flamegraph-rs](https://github.com/flamegraph-rs/flamegraph))
///
#[derive(Default, Debug, RUMDeJson, RUMSerJson, RUMWebTemplate)]
#[template(
    source = "
        <div class='f9'>
            <pre>
                {{ data }}
            </pre>
        </div>
    ",
    ext = "html"
)]
pub struct CPUBenchmarkReport {
    pub data: RUMString
}

impl TryFrom<&RUMBuffer> for CPUBenchmarkReport {
    type Error = RUMString;
    fn try_from(report: &RUMBuffer) -> Result<Self, Self::Error> {
        let data = buffer_to_string(report)?;
        Ok(Self {
            data
        })
    }
}
