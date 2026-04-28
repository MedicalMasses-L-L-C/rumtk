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
use rumtk_core::search::rumtk_search::{string_find_value, string_search};
use rumtk_core::strings::{rumtk_format, RUMString, RUMStringConversions};
use rumtk_core::types::{RUMDeserialize, RUMSerialize};
use rumtk_web::RUMWebTemplate;
use std::convert::{From, TryFrom};
use std::fmt::Debug;

///
/// Extracts basic benchmark information for later display. Note, this type should be paired with 
/// the output of `hyperfine`
/// 
/// ## Example
/// ```
/// let hyperfine_str = "Benchmark 1: ../target/debug/rumtk-hl7-v2-parse\n  Time (mean \xc2\xb1 \xcf\x83):       1.0 ms \xc2\xb1   0.6 ms    [User: 0.4 ms, System: 0.8 ms]\n  Range (min \xe2\x80\xa6 max):     0.5 ms \xe2\x80\xa6   7.4 ms    1474 runs\n \n| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |\n|:---|---:|---:|---:|---:|\n| `../target/debug/rumtk-hl7-v2-parse` | 1.0 \xc2\xb1 0.6 | 0.5 | 7.4 | 1.00 |\n";
/// BasicBenchmarkReport::try_from(hyperfine_str);
/// ```
/// 
#[derive(Default, Debug, RUMDeserialize, RUMSerialize, RUMWebTemplate)]
#[template(
    source = "
        <table>
            <thead>
                <th>Command</th>
                <th>Mean</th>
                <th>Std</th>
                <th>Min</th>
                <th>Max</th>
                <th>Relative</th>
                <th>User</th>
                <th>Kernel</th>
                <th>Runs</th>
                <th>Units</th>
            </thead>
            <tbody>
                <tr>
                    <td>{{command}}</td>
                    <td>{{mean_time}}</td>
                    <td>{{mean_delta}}</td>
                    <td>{{min}}</td>
                    <td>{{max}}</td>
                    <td>{{relative}}</td>
                    <td>{{user_time}}</td>
                    <td>{{kernel_time}}</td>
                    <td>{{runs}}</td>
                    <td>{{units}}</td>
                </tr>
            </tbody>
        </table>
    ",
    ext = "html"
)]
pub struct BasicBenchmarkReport {
    pub command: RUMString,
    pub units: RUMString,
    pub mean_time: f32,
    pub mean_delta: f32,
    pub min: f32,
    pub max: f32,
    pub relative: f32,
    pub user_time: f32,
    pub kernel_time: f32,
    pub runs: usize,
}

impl<'a> TryFrom<&'a str> for BasicBenchmarkReport {
    type Error = RUMString;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let collection = s
            .split('\n')
            .collect::<Vec<&str>>();

        if collection.len() <= 2 {
            println!("Nothing found to build report with! Input => {}", s);
            return Ok(Self::default());
        }

        let items = collection[collection.len() - 2..collection.len()].join("\n");
        let key_line = items
            .as_str()
            .split("|")
            .collect::<Vec<&str>>();

        match key_line.len() >= 5 {
            true => {
                let mean_values: Vec<&str> = match key_line.get(2) {
                    Some(data) => {
                        data.split("±").collect()
                    }
                    None => return Err(rumtk_format!("Issue parsing BasicBenchmarkReport. Input => {}", s))
                };

                let mean_time = mean_values.get(0).unwrap().trim().parse::<f32>().unwrap_or_default();
                let mean_delta = mean_values.get(1).unwrap().trim().parse::<f32>().unwrap_or_default();
                let user_time: f32 = string_find_value(s, &["User:.*?,", "\\d+.\\d+|\\d+"]).unwrap_or_default();
                let kernel_time: f32 = string_find_value(s, &["System:.*?]", "\\d+.\\d+|\\d+"]).unwrap_or_default();
                let runs: usize = string_find_value(s, &["\\d+ runs", "\\d+"]).unwrap_or_default();
                let units: RUMString = string_find_value(s, &["Mean \\[.*?\\]", "\\[.*?\\]"]).unwrap_or_default();

                Ok(Self {
                    command: key_line.get(1).unwrap().trim().to_rumstring(),
                    mean_time,
                    mean_delta,
                    min: key_line.get(3).unwrap().trim().parse::<f32>().unwrap_or_default(),
                    max: key_line.get(4).unwrap().trim().parse::<f32>().unwrap_or_default(),
                    relative: key_line.get(5).unwrap().trim().parse::<f32>().unwrap_or_default(),
                    runs,
                    user_time,
                    kernel_time,
                    units
                })
            },
            false => Err(rumtk_format!("Data is missing the key fields. You are probably missing --outout file flag in hyperfine. Got => {:?}", key_line))
        }
    }
}