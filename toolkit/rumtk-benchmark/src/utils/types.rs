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
use rumtk_core::core::RUMResult;
use rumtk_core::search::rumtk_search::string_search;
use rumtk_core::strings::{rumtk_format, RUMString, RUMStringConversions};
use rumtk_core::types::{RUMDeserialize, RUMSerialize};
use std::convert::{From, TryFrom};
use std::fmt::Debug;

#[derive(Default, Debug, RUMDeserialize, RUMSerialize)]
pub struct BenchmarkMeta {
    pub arch: RUMString,
    pub os: RUMString,
}

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
#[derive(Default, Debug, RUMDeserialize, RUMSerialize)]
pub struct BasicBenchmarkReport {
    pub command: RUMString,
    pub mean_time: usize,
    pub mean_delta: usize,
    pub min: usize,
    pub max: usize,
    pub relative: usize,
    pub user_time: usize,
    pub kernel_time: usize,
    pub runs: usize,
}

#[derive(Default, Debug, RUMDeserialize, RUMSerialize)]
pub struct BenchmarkReport<T> {
    pub meta: BenchmarkMeta,
    pub report: Option<T>,
}

impl BenchmarkMeta {
    pub fn new() -> RUMResult<Self> {
        Ok(Self {
            arch: RUMString::new(""),
            os: RUMString::new("")
        })
    }
}

impl<'a> TryFrom<&'a str> for BasicBenchmarkReport {
    type Error = RUMString;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let collection = s
            .split('\n')
            .collect::<Vec<&str>>();
        let items = collection[collection.len() - 2..collection.len()].join("\n");
        let key_line = items
            .as_str()
            .split("|")
            .collect::<Vec<&str>>();

        match key_line.len() >= 5 {
            true => {
                let mean_values: Vec<&str> = match key_line.get(1) {
                    Some(data) => {
                        data.split("±").collect()
                    }
                    None => return Err(rumtk_format!("Issue parsing BasicBenchmarkReport. Input => {}", s))
                };
                let mean_time = mean_values.get(0).unwrap().trim().parse::<usize>().unwrap_or_default();
                let mean_delta = mean_values.get(1).unwrap().trim().parse::<usize>().unwrap_or_default();
                let user_time = string_search(s, "User: \\d+.\\d+|\\d+", "").unwrap_or_default();
                let kernel_time = string_search(s, "System: \\d+.\\d+|\\d+", "").unwrap_or_default();
                let runs = string_search(s, "d runs", "").unwrap_or_default();

                Ok(Self {
                    command: key_line.get(0).unwrap().trim().to_rumstring(),
                    mean_time,
                    mean_delta,
                    min: key_line.get(2).unwrap().trim().parse::<usize>().unwrap_or_default(),
                    max: key_line.get(3).unwrap().trim().parse::<usize>().unwrap_or_default(),
                    relative: key_line.get(4).unwrap().trim().parse::<usize>().unwrap_or_default(),
                    runs: runs.trim().parse::<usize>().unwrap_or_default(),
                    user_time: user_time.trim().parse::<usize>().unwrap_or_default(),
                    kernel_time: kernel_time.trim().parse::<usize>().unwrap_or_default(),
                })
            },
            false => Err(rumtk_format!("Data is missing the key fields. Got => {}", key_line.len()))
        }
    }
}

impl<'a> TryFrom<&str> for BenchmarkReport<BasicBenchmarkReport> {
    type Error = RUMString;
    fn try_from(data: &str) -> Result<Self, Self::Error> 
    {
        Ok(Self {
            meta: BenchmarkMeta::new()?,
            report: Some(BasicBenchmarkReport::try_from(data)?),
        })
    }
}