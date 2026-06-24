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
use crate::utils::types::{RUMWebTemplate, ReportRawResults};
use rumtk_core::base::RUMResult;
use rumtk_core::buffers::buffer_to_string;
use rumtk_core::serde::{RUMDeJson, RUMSerJson};
use rumtk_core::strings::{rumtk_format, RUMString, RUMStringConversions};
use rumtk_core::types::RUMBuffer;
use std::convert::{From, TryFrom};
use std::env::consts;
use std::fmt::Debug;

#[derive(Default, Debug, RUMDeJson, RUMSerJson, RUMWebTemplate)]
#[template(
    source = "
        <table>
            <tbody>
                <tr>
                    <td>
                        <div class='f9'>
                            <pre>
                                {{ data }}
                            </pre>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                    <details>
                        <summary><strong><u>Test File Sizes in MB </u></strong></summary>
                        <ul>
                            {% for s in test_file_sizes.iter() %}
                            <li>{{s}}</li>
                            {% endfor %}
                        </ul>
                    </td>
                </tr>
            </tbody>
        </table>
        <div class='gap-10'></div>
    ",
    ext = "html"
)]
pub struct BenchmarkMeta {
    pub data: RUMString,
    pub test_file_sizes: Vec<f32>,
}

impl BenchmarkMeta {
    pub fn new() -> RUMResult<Self> {
        Ok(Self {
            data: RUMString::new(),
            test_file_sizes: vec![],
        })
    }
}

impl<'a> TryFrom<&RUMBuffer> for BenchmarkMeta {
    type Error = RUMString;
    fn try_from(data: &RUMBuffer) -> Result<Self, Self::Error>
    {
        Ok(Self {
            data: buffer_to_string(&data[..])?,
            test_file_sizes: vec![],
        })
    }
}