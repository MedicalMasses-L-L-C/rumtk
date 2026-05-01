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
use super::utils::{run_flamegraph, run_hyperfine, run_perf_report, FILE_SIZE_MB};
use crate::api::benchmarks::utils::generate_temp_dir;
use crate::api::benchmarks::utils::run_perf_stat;
use crate::utils::types::BenchmarkReport;
use rumtk_core::strings::{AsStr, RUMArrayConversions, RUMString, RUMStringConversions};
use rumtk_web::defaults::PARAMS_ID;
use rumtk_web::jobs::JobResult;
use rumtk_web::{rumtk_web_get_job_manager, rumtk_web_render_component, rumtk_web_render_page_contents, rumtk_web_render_template};
use rumtk_web::{APIPath, FormData, HTMLResult, RUMWebData, SharedAppState};

async fn basic_processor(form: FormData, state: SharedAppState) -> JobResult {
    let choice = match form.form.get("basic_choice"){
        Some(choice) => choice,
        None => &RUMString::default(),
    };
    let template = match form.form.get("basic_template"){
        Some(template) => template,
        None => &RUMString::default(),
    };

    let mut temp_data = generate_temp_dir()?;
    let pipeline_result = run_hyperfine(choice.as_str(), template.as_str(), &state, &mut temp_data).await?;
    let visualization = run_flamegraph(choice.as_str(), template.as_str(), &state, &mut temp_data).await?;
    let cpu_summary = run_perf_stat(choice.as_str(), "cpu_summary", template.as_str(), &state, &mut temp_data).await?;
    let cpu_cache = run_perf_report(choice.as_str(), "cpu_cache", template.as_str(), &state, &mut temp_data).await?;

    // Generate report
    let mut report = BenchmarkReport::try_from((&pipeline_result, &visualization, &cpu_summary, &cpu_cache))?;
    report.meta.test_file_sizes = temp_data.get_test_file_sizes::<FILE_SIZE_MB>()?;

    // Render the HTML result.
    Ok(Some(rumtk_web_render_template!(report)))
}

pub fn benchmark(_path: APIPath, _params: RUMWebData, form: FormData, state: SharedAppState) -> HTMLResult {
    let job_id = rumtk_web_get_job_manager!()?.spawn_task(basic_processor(form, state.clone()))?;
    let viewer = rumtk_web_render_component!("benchmark_view", [(PARAMS_ID, job_id)], state)?.to_rumstring();

    rumtk_web_render_page_contents!(
        &vec![
            viewer
        ]
    )
}
