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
use rumtk_core::core::{new_random_string_set, RUMVec, DEFAULT_BUFFER_CHUNK_SIZE, DEFAULT_BUFFER_ITEM_COUNT};
use rumtk_core::pipelines::pipeline_types::RUMCommandLine;
use rumtk_core::rumtk_pipeline_pipe_string_data;
use rumtk_core::strings::{string_format, CompactStringExt, RUMString, RUMStringConversions};
use rumtk_web::{rumtk_web_get_pipelines, SharedAppState, TextMap};

pub type RUMPipelineRuns = Vec<RUMCommandLine>;

pub fn generate_data(template: &str, buffer: &RUMVec<RUMString>, item_pattern: &str) -> RUMString {
    let mut lines: RUMVec<RUMString> = RUMVec::with_capacity(buffer.len());

    for i in 0..buffer.len() {
        let item = buffer.get(i).unwrap();
        lines.push(string_format(item_pattern,
                                 &[
                                     ("{line}", item),
                                     ("{line_number}", i.to_string().as_str())
                                 ]
        ));
    }

    let data = lines.join_compact("\n");

    template.replace("{data}", data.as_str()).to_rumstring()
}

pub fn generate_test_run(pipeline: &RUMCommandLine, settings: &TextMap) -> RUMCommandLine {
    // Generate the data.
    let random_data = new_random_string_set::<DEFAULT_BUFFER_CHUNK_SIZE>(DEFAULT_BUFFER_ITEM_COUNT * 2);
    let template = match settings.get("template") {
        Some(template) => template,
        None => &RUMString::default(),
    };
    let line_pattern = match settings.get("line_pattern") {
        Some(line_pattern) => line_pattern,
        None => &RUMString::default(),
    };
    let data = generate_data(template.as_str(), &random_data, line_pattern.as_str());

    // Prepare the pipeline
    let mut new_pipeline = pipeline.clone();
    rumtk_pipeline_pipe_string_data!(&mut new_pipeline, data.as_str());
    
    new_pipeline
}

pub fn generate_test_runs(pipeline_category: &str, pipeline_name: &str, state: &SharedAppState, count: usize) -> RUMPipelineRuns {
    let mut pipeline_runs = RUMPipelineRuns::with_capacity(count);
    // Grab settings
    let settings = match rumtk_web_get_pipelines!(state).get_settings() {
        Some(settings) => settings.clone(),
        None => TextMap::new()
    };
    let pipeline = rumtk_web_get_pipelines!(state).get_pipeline(pipeline_category, pipeline_name);

    // Generate a series of pipelines ready for testing.
    for i in 0..count {
        let new_pipeline = generate_test_run(&pipeline, &settings);
        pipeline_runs.push(new_pipeline);
    }

    pipeline_runs
}

