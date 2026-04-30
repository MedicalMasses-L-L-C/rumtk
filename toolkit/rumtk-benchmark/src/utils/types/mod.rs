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
use crate::utils::types::cpu_report::CPUBenchmarkReport;
use rumtk_core::strings::RUMString;
use rumtk_core::types::{RUMBuffer, RUMDeserialize, RUMSerialize};
use rumtk_web::RUMWebTemplate;
use std::convert::{From, TryFrom};
use std::fmt::Debug;

mod meta;
mod basic_report;
mod flamegraph;
mod cpu_report;

pub use basic_report::*;
pub use flamegraph::*;
pub use meta::*;

type ReportRawResults<'a> = (&'a RUMBuffer, &'a RUMBuffer, &'a RUMBuffer);

#[derive(Default, Debug, RUMDeserialize, RUMSerialize, RUMWebTemplate)]
#[template(
    source = "
        {{meta|safe}}
        {{report|safe}}
        {{visualization|safe}}
        {{cpu_summary|safe}}
    ",
    ext = "html"
)]
pub struct BenchmarkReport {
    pub meta: BenchmarkMeta,
    pub report: BasicBenchmarkReport,
    pub visualization: FlamegraphBenchmarkVisualizer,
    pub cpu_summary: CPUBenchmarkReport,
}

impl<'a> TryFrom<ReportRawResults<'a>> for BenchmarkReport {
    type Error = RUMString;
    fn try_from(data: ReportRawResults) -> Result<Self, Self::Error>
    {
        let (raw_report, raw_visualization, cpu_summary) = data;
        let report = BasicBenchmarkReport::try_from(raw_report)?;
        let visualization = FlamegraphBenchmarkVisualizer::try_from(raw_visualization)?;
        let cpu_summary = CPUBenchmarkReport::try_from(cpu_summary)?;
        println!("{:?}", &cpu_summary.render());
        Ok(Self {
            meta: BenchmarkMeta::new()?,
            report,
            visualization,
            cpu_summary,
        })
    }
}
