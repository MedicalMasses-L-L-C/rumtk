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
use rumtk_core::core::{new_random_string_set, RUMResult, RUMVec, DEFAULT_BUFFER_CHUNK_SIZE, DEFAULT_BUFFER_ITEM_COUNT};
use rumtk_core::pipelines::pipeline_types::RUMCommandLine;
use rumtk_core::strings::{rumtk_format, string_format, CompactStringExt, RUMString, RUMStringConversions};
use rumtk_core::types::RUMBuffer;
use rumtk_core::{rumtk_pipeline_patch_args, rumtk_pipeline_run_async};
use rumtk_web::{rumtk_web_get_pipelines, SharedAppState, TextMap};

use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use tempfile::{tempdir, NamedTempFile, TempDir};

pub const FILE_SIZE_KB: usize = 1024;
pub const FILE_SIZE_MB: usize = 1024 * 1024;

pub type RUMPipelineRuns = Vec<RUMCommandLine>;
pub type RUMPerfReport<'a> = (RUMBuffer, &'a mut NamedTempFile);

pub struct TempData {
    pub temp_dir: TempDir,
    pub test_files: Vec<NamedTempFile>,
    pub perf_files: Vec<NamedTempFile>,
}

impl TempData {
    pub fn get_test_file_sizes<const SIZE: usize>(&self) -> RUMResult<Vec<f32>> {
        let mut sizes = Vec::<f32>::with_capacity(self.test_files.len());

        for file in &self.test_files {
            let new_size = match fs::metadata(file.path().to_str().unwrap()) {
                Ok(metadata) => metadata.len() as f32 / SIZE as f32,
                Err(e) => return Err(rumtk_format!("Maybe a temp file is unexpectedly missing??? => {}", e)),
            };
            sizes.push(new_size);
        }

        Ok(sizes)
    }

    pub fn new_test_file(&mut self) -> RUMResult<&mut NamedTempFile> {
        let temp_file = match NamedTempFile::new_in(&self.temp_dir) {
            Ok(temp_file) => temp_file,
            Err(e) => return Err(rumtk_format!("Failed to create temporary test file because => {}", e))
        };
        self.test_files.push(temp_file);
        Ok(self.test_files.last_mut().unwrap())
    }

    pub fn new_perf_file(&mut self) -> RUMResult<&mut NamedTempFile> {
        let temp_file = match NamedTempFile::new_in(&self.temp_dir) {
            Ok(temp_file) => temp_file,
            Err(e) => return Err(rumtk_format!("Failed to create temporary perf file because => {}", e))
        };
        self.perf_files.push(temp_file);
        Ok(self.perf_files.last_mut().unwrap())
    }
}

pub fn read_temp_buffer(temp_file: &mut NamedTempFile) -> RUMResult<RUMBuffer> {
    let mut data = RUMVec::<u8>::new();

    match temp_file.seek(SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(e) => return Err(rumtk_format!("Failed to seek to start of temp file: {}", e)),
    };

    match temp_file.read_to_end(&mut data) {
        Ok(s) => s,
        Err(e) => return Err(rumtk_format!("Failed to read temp file contents => {}", e)),
    };

    Ok(RUMBuffer::copy_from_slice(data.as_slice()))
}

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

pub fn generate_test_run_data(settings: &TextMap) -> RUMString {
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
    generate_data(template.as_str(), &random_data, line_pattern.as_str())
}

pub fn generate_temp_test_run_data<'a>(temp_file: &'a mut NamedTempFile, settings: &TextMap) -> RUMResult<&'a NamedTempFile> {
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

    match temp_file.as_file().write(data.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(rumtk_format!("Failed to write temporary file because => {}", e))
    };

    Ok(temp_file)
}

pub fn generate_test_run<'a>(pipeline: &RUMCommandLine, settings: &TextMap, temp_file: &'a mut NamedTempFile) -> RUMResult<RUMCommandLine> {
    let mut new_pipeline = pipeline.clone();
    let temp_file = generate_temp_test_run_data(temp_file, settings)?;
    rumtk_pipeline_patch_args!(&mut new_pipeline, &[("{test_file}", temp_file.path().to_str().unwrap())]);

    Ok(new_pipeline)
}

pub fn get_settings(state: &SharedAppState) -> TextMap {
    match rumtk_web_get_pipelines!(state).get_settings() {
        Some(settings) => settings.clone(),
        None => TextMap::new()
    }
}

pub fn generate_test_runs(pipeline_category: &str, pipeline_name: &str, state: &SharedAppState, count: usize, temp: &mut TempData) -> RUMResult<RUMPipelineRuns> {
    let mut pipeline_runs = RUMPipelineRuns::with_capacity(count);
    // Grab settings
    let settings = get_settings(&state);
    let pipeline = rumtk_web_get_pipelines!(state).get_pipeline(pipeline_category, pipeline_name);

    // Generate a series of pipelines ready for testing.
    for i in 0..count {
        let new_pipeline = generate_test_run(&pipeline, &settings, temp.new_test_file()?);
        pipeline_runs.push(new_pipeline?);
    }

    Ok(pipeline_runs)
}

pub fn generate_temp_dir() -> RUMResult<TempData> {
    match tempdir() {
        Ok(dir) => Ok(TempData {
            temp_dir: dir,
            test_files: vec![],
            perf_files: vec![]
        }),
        Err(e) => Err(rumtk_format!("Failed to create temporary directory because => {}", e))
    }
}

pub async fn run_hyperfine(profile: &str, state: &SharedAppState, temp_data: &mut TempData) -> RUMResult<RUMBuffer> {
    let mut pipeline_runs = generate_test_runs("basic", "hyperfine", &state, 1, temp_data)?;
    let mut pipeline = pipeline_runs.first_mut().unwrap();
    let target = rumtk_web_get_pipelines!(state).get_target(profile);
    rumtk_pipeline_patch_args!(&mut pipeline, &[
        ("{target}", &target)
    ]);

    // Execute the pipeline
    Ok(rumtk_pipeline_run_async!(pipeline).await?)
}

pub async fn run_perf<'a>(command: &str, target: &str, state: &SharedAppState, temp_data: &'a mut TempData) -> RUMResult<RUMPerfReport<'a>> {
    let perf = rumtk_web_get_pipelines!(state).get_pipeline("perf", command);
    let settings = get_settings(&state);
    let mut run = generate_test_run(&perf, &settings, temp_data.new_perf_file()?)?;
    let mut perfdata = temp_data.new_perf_file()?;

    rumtk_pipeline_patch_args!(&mut run, &[
        ("{target}", &target),
        ("{perfdata}", &perfdata.path().to_str().unwrap_or_default())
    ]);

    // Execute the pipeline
    let results = rumtk_pipeline_run_async!(&run).await?;

    Ok((results, perfdata))
}

pub async fn run_perf_stat(profile: &str, command: &str, state: &SharedAppState, temp_data: &mut TempData) -> RUMResult<RUMBuffer> {
    let target = rumtk_web_get_pipelines!(state).get_target(profile);
    let (report, mut perfdata) = run_perf(command, &target, &state, temp_data).await?;

    read_temp_buffer(&mut perfdata)
}

pub async fn run_perf_report(profile: &str, command: &str, state: &SharedAppState, temp_data: &mut TempData) -> RUMResult<RUMBuffer> {
    let target = rumtk_web_get_pipelines!(state).get_target(profile);
    let (report, mut perfdata) = run_perf(command, &target, &state, temp_data).await?;
    let mut report_pipeline = rumtk_web_get_pipelines!(state).get_pipeline("visualizers", "perf");

    rumtk_pipeline_patch_args!(&mut report_pipeline, &[
        ("{perfdata}", &perfdata.path().to_str().unwrap_or_default())
    ]);

    // Execute the pipeline
    let vis_data = rumtk_pipeline_run_async!(&report_pipeline).await?;
    Ok(vis_data)
}

pub async fn run_flamegraph(profile: &str, state: &SharedAppState, temp_data: &mut TempData) -> RUMResult<RUMBuffer> {
    let target = rumtk_web_get_pipelines!(state).get_target(profile);
    let mut flamegraph = rumtk_web_get_pipelines!(state).get_pipeline("visualizers", "flamegraph");
    let (report, mut perfdata) = run_perf("perf", &target, &state, temp_data).await?;

    rumtk_pipeline_patch_args!(&mut flamegraph, &[
        ("{target}", &target),
        ("{perfdata}", &perfdata.path().to_str().unwrap_or_default())
    ]);

    // Execute the pipeline
    let vis_data = rumtk_pipeline_run_async!(&flamegraph).await?;
    Ok(vis_data)
}

