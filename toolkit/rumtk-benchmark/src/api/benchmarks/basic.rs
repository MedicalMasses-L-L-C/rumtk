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

use crate::api::benchmarks::utils::generate_test_runs;
use crate::utils::types::{BasicBenchmarkReport, BenchmarkReport};
use rumtk_core::rumtk_pipeline_quick_run_async;
use rumtk_core::strings::{rumtk_format, AsStr, RUMArrayConversions, RUMStringConversions};
use rumtk_web::defaults::PARAMS_ID;
use rumtk_web::jobs::JobResult;
use rumtk_web::{rumtk_web_get_job_manager, rumtk_web_render_component, rumtk_web_render_page_contents, rumtk_web_render_template};
use rumtk_web::{APIPath, FormData, HTMLResult, RUMWebData, SharedAppState};

async fn basic_processor(form: FormData, state: SharedAppState) -> JobResult {
    match form.form.get("basic_choice") {
        Some(pipeline_name) => {
            let pipeline_runs = generate_test_runs("basic", pipeline_name.as_str(), &state, 1);
            let pipeline = pipeline_runs.first().unwrap();

            // Execute the pipeline
            let result = rumtk_pipeline_quick_run_async!(pipeline).await?;

            // Generate report
            let report = match std::str::from_utf8(&result) {
                Ok(results) => BenchmarkReport::<BasicBenchmarkReport>::try_from(results)?,
                Err(e) => return Err(rumtk_format!("Invalid UTF-8 returned by pipeline: {}", e)),
            };

            // Render the HTML result.
            Ok(Some(rumtk_web_render_template!(report)))
        },
        None => Ok(None)
    }

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
