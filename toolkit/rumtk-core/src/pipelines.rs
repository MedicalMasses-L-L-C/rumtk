/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D.
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
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

pub mod pipeline_types {
    use crate::strings::RUMString;
    use crate::types::{RUMBuffer, RUMHashMap};

    use crate::core::{RUMResult, RUMVec};
    use std::process::{Child, Command};

    #[derive(Default, Debug, Clone)]
    pub struct RUMCommand {
        pub path: RUMString,
        pub args: Vec<RUMString>,
        pub env: RUMHashMap<RUMString, RUMString>,
    }
    pub type RUMCommandLine = RUMVec<RUMCommand>;
    pub type RUMPipelineCommand = Command;
    pub type RUMPipelineProcess = Child;
    pub type RUMPipeline = RUMVec<RUMPipelineProcess>;
    pub type RUMPipelineResult = RUMResult<RUMBuffer>;
}

pub mod pipeline_functions {
    use super::pipeline_types::*;
    use crate::core::RUMResult;
    use crate::strings::rumtk_format;
    use std::io::Read;

    use crate::threading::threading_functions::async_sleep;
    use crate::types::RUMBuffer;
    use std::process::{Command, Stdio};

    const DEFAULT_PROCESS_ASYNC_WAIT: f32 = 0.001;
    const DEFAULT_STDOUT_CHUNK_SIZE: usize = 1024;

    ///
    /// Given a command of type [RUMCommand](RUMCommand), generate a command instance the Rust
    /// runtime can use to spawn a process.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::any::{Any, TypeId};
    ///
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMPipelineCommand};
    /// use rumtk_core::pipelines::pipeline_functions::pipeline_generate_command;
    ///
    /// let command_name = "ls";
    /// let mut command = RUMCommand::default();
    /// command.path = RUMString::from(command_name);
    ///
    /// let sys_command = pipeline_generate_command(&command);
    ///
    /// assert_eq!(sys_command.get_program().to_str().unwrap(), command_name, "");
    ///
    /// ```
    ///
    pub fn pipeline_generate_command(command: &RUMCommand) -> RUMPipelineCommand {
        let mut cmd = Command::new(command.path.as_str());

        for arg in command.args.iter() {
            cmd.arg(arg);
        }

        cmd.envs(command.env.iter());

        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped());

        cmd
    }

    ///
    /// Spawns a process out of the [RUMPipelineCommand](RUMPipelineCommand).
    ///
    /// ## Example
    ///
    /// ```
    /// use std::any::{Any, TypeId};
    ///
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMPipelineCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_command, pipeline_spawn_process};
    ///
    /// let command_name = "ls";
    /// let mut command = RUMCommand::default();
    /// command.path = RUMString::from(command_name);
    ///
    /// let mut sys_command = pipeline_generate_command(&command);
    ///
    /// let mut process = pipeline_spawn_process(&mut sys_command).unwrap();
    ///
    /// process.wait();
    /// ```
    ///
    pub fn pipeline_spawn_process(cmd: &mut RUMPipelineCommand) -> RUMResult<RUMPipelineProcess> {
        match cmd.spawn() {
            Ok(process) => Ok(process),
            Err(e) => Err(rumtk_format!(
                "Failed to spawn process {:?} because => {}",
                cmd.get_program(),
                e
            )),
        }
    }

    ///
    /// Given a process of type [RUMPipelineProcess](RUMPipelineProcess) and a rhs of type [RUMPipelineCommand](RUMPipelineCommand),
    /// create a pipe of the lhs's `stdout` into the next command descriptor which is the rhs.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::any::{Any, TypeId};
    /// use std::process::Stdio;
    ///
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMPipelineCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_command, pipeline_pipe_process, pipeline_spawn_process};
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_generate_command(&ls_command);
    ///
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    /// let mut sys_wc_command = pipeline_generate_command(&wc_command);
    ///
    /// let mut sys_ls_process = pipeline_spawn_process(&mut sys_ls_command).unwrap();
    /// pipeline_pipe_process(&mut sys_ls_process, &mut sys_wc_command).unwrap();
    /// let mut sys_wc_process = pipeline_spawn_process(&mut sys_wc_command).unwrap();
    ///
    /// sys_ls_process.wait();
    /// sys_wc_process.wait();
    /// ```
    ///
    pub fn pipeline_pipe_process(
        process: &mut RUMPipelineProcess,
        piped: &mut RUMPipelineCommand,
    ) -> RUMResult<()> {
        let process_stdout = Stdio::from(match process.stdout.take() {
            Some(stdout) => stdout,
            None => {
                return Err(rumtk_format!(
                    "No stdout handle found for process {}.",
                    process.id()
                ));
            }
        });
        let _ = piped.stdin(process_stdout);
        Ok(())
    }

    ///
    /// Retrieves the standard output generated by the completed process.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::any::{Any, TypeId};
    /// use std::process::Stdio;
    ///
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMPipelineCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_command, pipeline_pipe_process, pipeline_spawn_process, pipeline_get_stdout};
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_generate_command(&ls_command);
    ///
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    /// let mut sys_wc_command = pipeline_generate_command(&wc_command);
    ///
    /// let mut sys_ls_process = pipeline_spawn_process(&mut sys_ls_command).unwrap();
    /// pipeline_pipe_process(&mut sys_ls_process, &mut sys_wc_command).unwrap();
    /// let mut sys_wc_process = pipeline_spawn_process(&mut sys_wc_command).unwrap();
    ///
    /// sys_ls_process.wait();
    /// sys_wc_process.wait();
    ///
    /// let mut pipeline = vec![sys_ls_process, sys_wc_process];
    ///
    /// let out_data = pipeline_get_stdout(pipeline).unwrap();
    ///
    /// assert_eq!(out_data.is_empty(), false, "No output detected... {:?}", &out_data);
    /// ```
    ///
    pub fn pipeline_get_stdout(mut pipeline: RUMPipeline) -> RUMResult<RUMBuffer> {
        let mut last_item = pipeline.pop().unwrap();
        match last_item.wait_with_output() {
            Ok(stdout) => Ok(RUMBuffer::from(stdout.stdout.clone())),
            Err(e) => Err(rumtk_format!(
                "Issue reading last process output because => {}",
                e
            )),
        }
    }

    pub fn pipeline_connect_processes<'a>(
        root: &'a mut RUMPipelineCommand,
        piped: &'a mut RUMPipelineCommand,
    ) -> RUMResult<RUMPipelineProcess> {
        let mut root_process = pipeline_spawn_process(root)?;
        pipeline_pipe_process(&mut root_process, piped)?;
        Ok(root_process)
    }

    ///
    /// Builds an executable pipeline out of a list of [RUMCommand](RUMCommand).
    ///
    /// ## Example
    ///
    /// ```
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_pipeline};
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    ///
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    ///
    /// let commands = vec![
    ///     ls_command,
    ///     wc_command
    /// ];
    ///
    /// let pipeline = pipeline_generate_pipeline(&commands).unwrap();
    ///
    /// assert_eq!(pipeline.len(), commands.len(), "Pipeline generation returned unexpected number of items!");
    /// ```
    ///
    pub fn pipeline_generate_pipeline(commands: &RUMCommandLine) -> RUMResult<RUMPipeline> {
        let mut root = pipeline_generate_command(commands.first().unwrap());
        let mut parent_process;

        // Setup pipeline
        let mut pipeline = vec![];
        root.stdin(Stdio::piped()).stdout(Stdio::piped());

        for cmd in commands.iter().skip(1) {
            let mut new_root = pipeline_generate_command(cmd);
            parent_process = pipeline_connect_processes(&mut root, &mut new_root)?;
            pipeline.push(parent_process);
            root = new_root;
        }

        pipeline.push(pipeline_spawn_process(&mut root)?);

        Ok(pipeline)
    }

    ///
    /// Await for pipeline to execute in a async friendly manner. Once the pipeline execution ends,
    /// consume the pipeline and return the output.
    ///
    /// ## Example
    ///
    /// ```
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_pipeline, pipeline_await_pipeline};
    /// use rumtk_core::{rumtk_resolve_task, rumtk_init_threads};
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    ///
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    ///
    /// let commands = vec![
    ///     ls_command,
    ///     wc_command
    /// ];
    ///
    /// let pipeline = pipeline_generate_pipeline(&commands).unwrap();
    /// let rt = rumtk_init_threads!(&5);
    /// let result = rumtk_resolve_task!(rt, pipeline_await_pipeline(pipeline)).unwrap();
    ///
    /// assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    /// ```
    ///
    pub async fn pipeline_await_pipeline(mut pipeline: RUMPipeline) -> RUMPipelineResult {
        for p in pipeline.iter_mut() {
            loop {
                match p.try_wait() {
                    Ok(code) => match code {
                        Some(code) => {
                            if !code.success() {
                                return Err(rumtk_format!(
                                    "Process {} exited with non-success code => {}!",
                                    p.id(),
                                    code
                                ));
                            }
                            break;
                        }
                        None => {
                            async_sleep(DEFAULT_PROCESS_ASYNC_WAIT).await;
                            continue;
                        }
                    },
                    Err(e) => return Err(rumtk_format!("Issue with process {} => {}", p.id(), e)),
                };
            }
        }

        let result = pipeline_get_stdout(pipeline)?;
        Ok(result)
    }

    ///
    /// Await for pipeline to complete execution. Once the pipeline execution ends,
    /// consume the pipeline and return the output.
    ///
    /// ## Example
    ///
    /// ```
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_pipeline, pipeline_wait_pipeline};
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    ///
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    ///
    /// let commands = vec![
    ///     ls_command,
    ///     wc_command
    /// ];
    ///
    /// let pipeline = pipeline_generate_pipeline(&commands).unwrap();
    /// let result = pipeline_wait_pipeline(pipeline).unwrap();
    ///
    /// assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    /// ```
    ///
    pub fn pipeline_wait_pipeline(mut pipeline: RUMPipeline) -> RUMPipelineResult {
        for p in pipeline.iter_mut() {
            match p.wait() {
                Ok(code) => {
                    if !code.success() {
                        return Err(rumtk_format!(
                            "Process {} exited with non-success code => {}!",
                            p.id(),
                            code
                        ));
                    }
                    break;
                }
                Err(e) => return Err(rumtk_format!("Issue with process {} => {}", p.id(), e)),
            };
        }

        let result = pipeline_get_stdout(pipeline)?;
        Ok(result)
    }
}
