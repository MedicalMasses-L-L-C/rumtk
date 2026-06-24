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

pub type MetaData = (RUMBuffer,RUMBuffer,RUMBuffer);

#[derive(Default, Debug, RUMDeJson, RUMSerJson, RUMWebTemplate)]
#[template(
    source = "
        <table>
            <tbody>
                <tr><td><h3>Architecture</h3></td></tr>
                <tr>
                    <td>
                        <div class='f9'>
                            <pre>
                                {{ cpu_info }}
                            </pre>
                        </div>
                    </td>
                </tr>
                <tr><td><h3>Available Metrics</h3></td></tr>
                <tr>
                    <td>
                        <div class='f9'>
                            <pre>
                                {{ cpu_hw_metrics }}
                            </pre>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class='f9'>
                            <pre>
                                {{ cpu_cache_metrics }}
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
    pub cpu_info: RUMString,
    pub cpu_hw_metrics: RUMString,
    pub cpu_cache_metrics: RUMString,
    pub test_file_sizes: Vec<f32>,
}

impl BenchmarkMeta {
    pub fn new() -> RUMResult<Self> {
        Ok(Self {
            cpu_info: RUMString::new(),
            cpu_hw_metrics: RUMString::new(),
            cpu_cache_metrics: RUMString::new(),
            test_file_sizes: vec![],
        })
    }
}

impl<'a> TryFrom<MetaData> for BenchmarkMeta {
    type Error = RUMString;
    fn try_from(data: MetaData) -> Result<Self, Self::Error>
    {
        let (cpu_info, cpu_hw_metrics, cpu_cache_metrics) = data;
        Ok(Self {
            cpu_info: buffer_to_string(&cpu_info[..])?,
            cpu_hw_metrics: buffer_to_string(&cpu_hw_metrics[..])?,
            cpu_cache_metrics: buffer_to_string(&cpu_cache_metrics[..])?,
            test_file_sizes: vec![],
        })
    }
}