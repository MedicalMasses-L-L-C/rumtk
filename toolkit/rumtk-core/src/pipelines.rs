/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod pipeline_types {
    use crate::core::{RUMResult, RUMVec};
    use crate::strings::{RUMString, RUMStringConversions};
    use crate::types::{RUMBuffer, RUMDeserialize, RUMHashMap, RUMSerialize};
    use std::process::{Child, Command};

    pub static EMPTY_COMMAND_LINE: RUMCommandLine = RUMCommandLine::new();

    pub type RUMCommandArgs = Vec<RUMString>;
    pub type RUMCommandEnv = RUMHashMap<RUMString, RUMString>;
    #[derive(RUMSerialize, RUMDeserialize, PartialEq, Default, Debug, Clone)]
    pub struct RUMCommand {
        pub path: RUMString,
        #[serde(skip)]
        pub data: Option<RUMBuffer>,
        pub args: RUMCommandArgs,
        pub env: RUMCommandEnv,
    }

    impl RUMCommand {
        pub fn new(
            prog: &str,
            data: &Option<RUMBuffer>,
            args: &RUMCommandArgs,
            env: &RUMCommandEnv,
        ) -> Self {
            RUMCommand {
                path: prog.to_rumstring(),
                args: args.clone(),
                env: env.clone(),
                data: data.clone(),
            }
        }
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
    use crate::strings::{rumtk_format, string_format, CompactStringExt, RUMArrayConversions, RUMString, StringReplacementPair};
    use std::io::{Read, Write};
    use std::os::unix::ffi::OsStrExt;

    use crate::rumtk_resolve_sync_task;
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
    /// use rumtk_core::pipelines::pipeline_functions::pipeline_create_command;
    ///
    /// let command_name = "ls";
    /// let mut command = RUMCommand::default();
    /// command.path = RUMString::from(command_name);
    ///
    /// let sys_command = pipeline_create_command(&command);
    ///
    /// assert_eq!(sys_command.get_program().to_str().unwrap(), command_name, "");
    ///
    /// ```
    ///
    pub fn pipeline_create_command(command: &RUMCommand) -> RUMPipelineCommand {
        let mut cmd = Command::new(command.path.as_str());

        for arg in command.args.iter() {
            cmd.arg(arg);
        }

        cmd.envs(command.env.iter());

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

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
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_create_command, pipeline_spawn_process};
    ///
    /// let command_name = "ls";
    /// let mut command = RUMCommand::default();
    /// command.path = RUMString::from(command_name);
    ///
    /// let mut sys_command = pipeline_create_command(&command);
    ///
    /// let mut process = pipeline_spawn_process(&mut sys_command).unwrap();
    ///
    /// process.wait();
    /// ```
    ///
    pub fn pipeline_spawn_process(cmd: &mut RUMPipelineCommand) -> RUMResult<RUMPipelineProcess> {
        match cmd.spawn() {
            Ok(process) => {
                println!("Spawned process {} => {} with args {:?}", process.id(), cmd.get_program().as_bytes().to_rumstring(), cmd.get_args());
                Ok(process)
            },
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
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_create_command, pipeline_pipe_processes, pipeline_spawn_process};
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_create_command(&ls_command);
    ///
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    /// let mut sys_wc_command = pipeline_create_command(&wc_command);
    ///
    /// let mut sys_ls_process = pipeline_spawn_process(&mut sys_ls_command).unwrap();
    /// pipeline_pipe_processes(&mut sys_ls_process, &mut sys_wc_command).unwrap();
    /// let mut sys_wc_process = pipeline_spawn_process(&mut sys_wc_command).unwrap();
    ///
    /// sys_ls_process.wait();
    /// sys_wc_process.wait();
    /// ```
    ///
    pub fn pipeline_pipe_processes(
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
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_create_command, pipeline_spawn_process, pipeline_get_stdout, pipeline_pipe_processes};
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_create_command(&ls_command);
    ///
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    /// let mut sys_wc_command = pipeline_create_command(&wc_command);
    ///
    /// let mut sys_ls_process = pipeline_spawn_process(&mut sys_ls_command).unwrap();
    /// pipeline_pipe_processes(&mut sys_ls_process, &mut sys_wc_command).unwrap();
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
    pub fn pipeline_get_stdout(mut process: RUMPipelineProcess) -> RUMResult<RUMBuffer> {
        match process.wait_with_output() {
            Ok(stdout) => Ok(RUMBuffer::from(stdout.stdout.clone())),
            Err(e) => Err(rumtk_format!(
                "Issue reading last process output because => {}",
                e
            )),
        }
    }

    ///
    /// Closes the `stdin` standard in file for process. Useful to trigger a resolution of the pipeline.
    ///
    /// ## Example
    ///
    /// ```
    /// use rumtk_core::pipelines::pipeline_functions::pipeline_close_process_stdin;
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMPipelineCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_create_command, pipeline_pipe_into_process, pipeline_spawn_process};
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_create_command(&ls_command);
    /// let mut sys_ls_process = pipeline_spawn_process(&mut sys_ls_command).unwrap();
    ///
    /// pipeline_close_process_stdin(&mut sys_ls_process);
    ///
    ///
    /// ```
    ///
    pub fn pipeline_close_process_stdin(process: &mut RUMPipelineProcess) {
        // Do not change into an expect() or such unwrap. We just want to ignore and assume stdin is closed.
        match process.stdin.take() {
            Some(stdin) => {
                drop(stdin);
            }
            None => {}
        };
    }

    ///
    /// Pipe data into a process.
    ///
    /// ## Example
    ///
    /// ```
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMPipelineCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_create_command, pipeline_pipe_into_process, pipeline_spawn_process};
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_create_command(&ls_command);
    /// let mut sys_ls_process = pipeline_spawn_process(&mut sys_ls_command).unwrap();
    /// pipeline_pipe_into_process(&mut sys_ls_process, &Some(RUMBuffer::default())).unwrap();
    ///
    /// let out = sys_ls_process.wait_with_output().unwrap();
    ///
    /// assert_eq!(out.stdout.is_empty(), false, "Piped command returned an empty buffer? => {:?}", String::from_utf8_lossy(out.stdout.as_slice()))
    /// ```
    ///
    pub fn pipeline_pipe_into_process(
        process: &mut RUMPipelineProcess,
        data: &RUMBuffer,
    ) -> RUMResult<()> {
            match process.stdin.take() {
                Some(ref mut stdin) => match stdin.write_all(data.as_slice()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(rumtk_format!(
                            "Failed to pipe data to stdin of process {} because => {}",
                            process.id(),
                            e
                        ))
                    }
                },
                None => {}
            }
        Ok(())
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
    pub fn pipeline_generate_command(command: &RUMCommand, data: &RUMBuffer) -> RUMResult<RUMPipelineProcess> {
        let mut root = pipeline_create_command(&command);
        let mut process = pipeline_spawn_process(&mut root)?;
        pipeline_pipe_into_process(&mut process, data)?;
        pipeline_close_process_stdin(&mut process);

        Ok(process)
    }

    ///
    /// Add buffer at the beginning of pipeline to pipe in. This buffer serves as the initial input of
    /// a pipe aware program.
    ///
    /// ## Example
    /// ```
    /// use rumtk_core::core::RUMResult;
    /// use rumtk_core::pipelines::pipeline_functions::pipeline_add_stdin_data_to_pipeline;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMCommandLine};
    /// use rumtk_core::rumtk_pipeline_quick_run;
    /// use rumtk_core::strings::{buffer_to_string, RUMString};
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let data = RUMBuffer::from_static(b"Hello World");
    /// let wc_name = "wc";
    /// let mut wc_command = RUMCommand::default();
    /// wc_command.path = RUMString::from(wc_name);
    /// let mut pipeline = vec![
    ///     wc_command
    /// ];
    ///
    /// pipeline_add_stdin_data_to_pipeline(&mut pipeline, &data);
    ///
    /// let processor = || -> RUMResult<RUMBuffer> {rumtk_pipeline_quick_run!(pipeline)};
    /// let result_string = buffer_to_string(&processor().unwrap()).unwrap();
    /// let binding = result_string.as_str().replace('\n', "");
    /// let result_items: Vec<&str> = binding.split("      ").collect();
    /// let result = result_items.get(2).unwrap().trim().parse::<i32>().unwrap();
    ///
    /// assert_eq!(result, 2, "Data was not piped properly!");
    /// ```
    ///
    pub fn pipeline_add_stdin_data_to_pipeline<'a>(pipeline: &'a mut RUMCommandLine, data: &RUMBuffer) -> &'a RUMCommandLine {
        match pipeline.get_mut(0) {
            Some(command) => command.data = Some(data.clone()),
            None => {
                return pipeline;
            }
        };

        pipeline
    }

    ///
    /// Flatten the [RUMCommandLine] structure into a single string representing the pipeline and
    /// print it or log it.
    ///
    fn print_pipeline(pipeline: &RUMCommandLine) {
        let mut pipeline_components = Vec::<RUMString>::with_capacity(pipeline.len());

        for pipe in pipeline.iter() {
            pipeline_components.push(rumtk_format!("{} {}", pipe.path, pipe.args.clone().join_compact(" ")));
        }

        println!("Executing {}", pipeline_components.join_compact(" | "));
    }

    pub fn pipeline_patch_command_args<'a>(cmd: &'a mut RUMCommand, replacements: &StringReplacementPair) -> RUMResult<&'a RUMCommand> {
        let mut new_args = RUMCommandArgs::with_capacity(cmd.args.len());

        for arg in cmd.args.iter() {
            new_args.push(string_format(arg, replacements));
        }

        cmd.args = new_args;

        Ok(cmd)
    }

    ///
    /// Patches the arguments of the first command with the pattern=replacement pairs!
    ///
    /// ## Example
    /// ```
    /// use rumtk_core::core::RUMResult;
    /// use rumtk_core::pipelines::pipeline_functions::pipeline_patch_args;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMCommandLine};
    /// use rumtk_core::rumtk_pipeline_quick_run;
    /// use rumtk_core::strings::{buffer_to_string, RUMString};
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// ls_command.args.push(RUMString::from("{options}"));
    /// let mut pipeline = vec![
    ///     ls_command
    /// ];
    /// pipeline_patch_args(&mut pipeline, &[("{options}", "-la")]);
    /// println!("{:#?}", pipeline);
    ///
    /// let processor = || -> RUMResult<RUMBuffer> {rumtk_pipeline_quick_run!(pipeline)};
    /// let result_string = buffer_to_string(&processor().unwrap()).unwrap();
    /// let results: Vec<&str> = result_string.as_str().split("\n").collect();
    /// let dot_dir = results.get(1).unwrap().chars().last().unwrap();
    ///
    /// assert_eq!(dot_dir, '.', "Incorrect options passed!");
    /// ```
    ///
    pub fn pipeline_patch_args<'a>(pipeline: &'a mut RUMCommandLine, replacements: &StringReplacementPair) -> RUMResult<&'a RUMCommandLine> {
        for mut cmd in pipeline.iter_mut() {
            pipeline_patch_command_args(&mut cmd, replacements)?;
        }
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
    /// use rumtk_core::{rumtk_resolve_task};
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
    /// let result = rumtk_resolve_task!(pipeline_await_pipeline(pipeline)).unwrap();
    ///
    /// assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    /// ```
    ///
    pub async fn pipeline_await_pipeline(pipeline: &RUMCommandLine, initial_data: &RUMBuffer) -> RUMPipelineResult {
        let pipeline_copy = pipeline.clone();
        let data_copy = initial_data.clone();
        rumtk_resolve_sync_task!(move || {
            pipeline_wait_pipeline(&pipeline_copy, &data_copy)
        })
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
    pub fn pipeline_wait_pipeline(pipeline: &RUMCommandLine, initial_data: &RUMBuffer) -> RUMPipelineResult {
        let mut last_data = initial_data.clone();

        // Now let's visit each process and await their completion!
        for c in pipeline.iter() {
            let mut p = pipeline_generate_command(&c, &last_data)?;
            pipeline_close_process_stdin(&mut p);
            println!("Waiting on {}", p.id());
            last_data = pipeline_get_stdout(p)?;
        }

        Ok(last_data)
    }
}

pub mod pipeline_macros {
    ///
    /// Creates a pipeline command out of the provided parameters. Parameters include `path`, `args`,
    /// and `env`. The command has [RUMCommand](super::pipeline_types::RUMCommand).
    ///
    /// `env` is a map of type [RUMCommandEnv](super::pipeline_types::RUMCommandEnv) containing a set of
    /// key value pair strings that we can use to update the process environment.
    ///
    /// ## Example
    ///
    /// ### Program Only
    ///
    /// ```
    /// use rumtk_core::rumtk_pipeline_command;
    ///
    /// let command = rumtk_pipeline_command!("ls");
    /// ```
    ///
    /// ### Program with Piped Data
    ///
    /// ```
    /// use rumtk_core::rumtk_pipeline_command;
    /// use rumtk_core::types::RUMBuffer;
    /// use rumtk_core::strings::RUMStringConversions;
    ///
    /// let command = rumtk_pipeline_command!("ls", RUMBuffer::default());
    /// ```
    ///
    /// ### Program with Args
    ///
    /// ```
    /// use rumtk_core::rumtk_pipeline_command;
    /// use rumtk_core::types::RUMBuffer;
    /// use rumtk_core::strings::RUMStringConversions;
    ///
    /// let command = rumtk_pipeline_command!("ls", RUMBuffer::default(), &vec![
    ///     "-l".to_rumstring()
    /// ]);
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_pipeline_command {
        ( $path:expr, $data:expr, $args:expr, $env:expr ) => {{
            use $crate::pipelines::pipeline_types::RUMCommand;

            RUMCommand::new($path, &Some($data), $args, $env)
        }};
        ( $path:expr, $data:expr, $args:expr ) => {{
            use $crate::pipelines::pipeline_types::{RUMCommand, RUMCommandEnv};

            RUMCommand::new($path, &Some($data), $args, &RUMCommandEnv::default())
        }};
        ( $path:expr, $data:expr ) => {{
            use $crate::pipelines::pipeline_types::{RUMCommand, RUMCommandArgs, RUMCommandEnv};

            RUMCommand::new(
                $path,
                &Some($data),
                &RUMCommandArgs::default(),
                &RUMCommandEnv::default(),
            )
        }};
        ( $path:expr ) => {{
            use $crate::pipelines::pipeline_types::{RUMCommand, RUMCommandArgs, RUMCommandEnv};
            use $crate::types::RUMBuffer;

            RUMCommand::new(
                $path,
                &None,
                &RUMCommandArgs::default(),
                &RUMCommandEnv::default(),
            )
        }};
    }

    ///
    /// Given a series of [RUMCommand](super::pipeline_types::RUMCommand) passed to this macro, prepare
    /// and execute the commands in a pipeline. A pipeline here refers to the Unix style pipeline which is the
    /// terminal form of a pipeline. The pipeline behaves like it would in the terminal => `ls | wc`.
    ///
    /// See [this article](https://cscie26.dce.harvard.edu/~dce-lib113/reference/unix/unix2.html)
    /// and the [Unix Philosophy](https://cscie2x.dce.harvard.edu/hw/ch01s06.html) to learn more!
    ///
    /// ## Example
    ///
    /// ### Simple
    ///
    /// ```
    /// use rumtk_core::{rumtk_pipeline_command, rumtk_pipeline_run, rumtk_resolve_task, rumtk_init_threads};
    /// use rumtk_core::core::{RUMResult};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = async || -> RUMResult<()> {
    ///     let result = rumtk_pipeline_run!(
    ///         rumtk_pipeline_command!("ls"),
    ///         rumtk_pipeline_command!("wc")
    ///     ).unwrap();
    ///
    ///     assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    ///     Ok(())
    /// };
    ///
    /// rumtk_resolve_task!(f()).unwrap();
    /// ```
    ///
    /// ### With Buffer Piped In
    ///
    /// ```
    /// use rumtk_core::{rumtk_pipeline_command, rumtk_pipeline_run, rumtk_resolve_task, rumtk_init_threads};
    /// use rumtk_core::core::{RUMResult, new_random_rumbuffer};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = || -> RUMResult<()> {
    ///     let result = rumtk_pipeline_run!(
    ///         rumtk_pipeline_command!("ls", new_random_rumbuffer::<0>()),
    ///         rumtk_pipeline_command!("wc")
    ///     )?;
    ///
    ///     assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    ///     Ok(())
    /// };
    ///
    /// f().unwrap();
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_pipeline_run {
        ( $($command:expr),+ ) => {{
            use $crate::pipelines::pipeline_functions::{pipeline_generate_pipeline, pipeline_wait_pipeline};

            let pipeline = pipeline_generate_pipeline(&vec![$($command),+])?;

            pipeline_wait_pipeline(pipeline)
        }};
    }

    ///
    /// Given a series of [RUMCommand](super::pipeline_types::RUMCommand) passed to this macro, prepare
    /// and execute the commands in a pipeline. A pipeline here refers to the Unix style pipeline which is the
    /// terminal form of a pipeline. The pipeline behaves like it would in the terminal => `ls | wc`.
    ///
    /// See [this article](https://cscie26.dce.harvard.edu/~dce-lib113/reference/unix/unix2.html)
    /// and the [Unix Philosophy](https://cscie2x.dce.harvard.edu/hw/ch01s06.html) to learn more!
    ///
    /// This is the `async` flavor.
    ///
    /// ## Example
    ///
    /// ### Simple
    ///
    /// ```
    /// use rumtk_core::{rumtk_pipeline_command, rumtk_pipeline_run_async, rumtk_resolve_task, rumtk_init_threads};
    /// use rumtk_core::core::{RUMResult};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = async || -> RUMResult<()> {
    ///     let result = rumtk_pipeline_run_async!(
    ///         rumtk_pipeline_command!("ls"),
    ///         rumtk_pipeline_command!("wc")
    ///     ).await?;
    ///
    ///     assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    ///     Ok(())
    /// };
    ///
    /// rumtk_resolve_task!(f()).unwrap();
    /// ```
    ///
    /// ### With Buffer Piped In
    ///
    /// ```
    /// use rumtk_core::{rumtk_pipeline_command, rumtk_pipeline_run_async, rumtk_resolve_task, rumtk_init_threads};
    /// use rumtk_core::core::{RUMResult, new_random_rumbuffer};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = async || -> RUMResult<()> {
    ///     let result = rumtk_pipeline_run_async!(
    ///         rumtk_pipeline_command!("wc", new_random_rumbuffer::<0>())
    ///     ).await?;
    ///
    ///     assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    ///     Ok(())
    /// };
    ///
    /// rumtk_resolve_task!(f()).unwrap();
    /// ```
    ///
    /// ### With Buffer Piped In W/ Return
    ///
    /// ```
    /// use rumtk_core::{rumtk_pipeline_command, rumtk_pipeline_run_async, rumtk_resolve_task, rumtk_init_threads};
    /// use rumtk_core::core::{RUMResult, new_random_rumbuffer, DEFAULT_BUFFER_CHUNK_SIZE};
    /// use rumtk_core::strings::{RUMString, RUMStringConversions, RUMArrayConversions};
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let expected = "1024\n";
    ///
    /// let f = async || -> RUMResult<RUMBuffer> {
    ///     let result = rumtk_pipeline_run_async!(
    ///         rumtk_pipeline_command!("wc", new_random_rumbuffer::<DEFAULT_BUFFER_CHUNK_SIZE>())
    ///     ).await?;
    ///
    ///     Ok(result)
    /// };
    ///
    /// let result = rumtk_resolve_task!(f()).unwrap();
    /// let string = result.to_vec().to_rumstring();
    /// let result_buffer_size = string.split("      ").last().unwrap().split("    ").last().unwrap().to_rumstring();
    ///
    /// assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    /// assert_eq!(&result_buffer_size, expected, "Pipeline returned an unexpected result from command wc! => {:?}\nvs.\n{:?}", &result_buffer_size, &expected);
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_pipeline_run_async {
        ( $($command:expr),+ ) => {{
            use $crate::pipelines::pipeline_functions::{pipeline_generate_pipeline, pipeline_await_pipeline};

            let pipeline = pipeline_generate_pipeline(&vec![$($command),+])?;

            pipeline_await_pipeline(pipeline)
        }};
    }

    ///
    /// This macro is similar to [rumtk_pipeline_run](crate::rumtk_pipeline_run). The difference here 
    /// is that the function takes a pipeline structure ([RUMCommandLine](crate::pipelines::pipeline_types::RUMCommandLine))
    /// In other words, this macro simply runs an already defined pipeline.
    /// 
    /// ## Example
    /// ```
    /// use rumtk_core::{rumtk_pipeline_command, rumtk_pipeline_quick_run, rumtk_resolve_task, rumtk_init_threads};
    /// use rumtk_core::core::{RUMResult};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = || -> RUMResult<RUMBuffer> {
    ///     let pipeline = vec![
    ///         rumtk_pipeline_command!("ls"),
    ///         rumtk_pipeline_command!("wc")
    ///     ];
    /// 
    ///     rumtk_pipeline_quick_run!(pipeline)
    /// };
    /// 
    /// f().unwrap();
    /// ```
    /// 
    #[macro_export]
    macro_rules! rumtk_pipeline_quick_run {
        ( $pipeline:expr ) => {{
            use $crate::pipelines::pipeline_functions::{pipeline_generate_pipeline, pipeline_wait_pipeline};

            let pipeline = pipeline_generate_pipeline(&$pipeline)?;

            pipeline_wait_pipeline(pipeline)
        }};
    }


    ///
    /// This macro is similar to [rumtk_pipeline_run_async](crate::rumtk_pipeline_run_async). The difference here 
    /// is that the function takes a pipeline structure ([RUMCommandLine](crate::pipelines::pipeline_types::RUMCommandLine))
    /// In other words, this macro simply runs an already defined pipeline.
    ///
    /// ## Example
    /// ```
    /// use rumtk_core::{rumtk_pipeline_command, rumtk_pipeline_quick_run_async, rumtk_resolve_task, rumtk_init_threads};
    /// use rumtk_core::core::{RUMResult};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = async || -> RUMResult<RUMBuffer> {
    ///     let pipeline = vec![
    ///         rumtk_pipeline_command!("ls"),
    ///         rumtk_pipeline_command!("wc")
    ///     ];
    ///
    ///     rumtk_pipeline_quick_run_async!(pipeline).await
    /// };
    ///
    /// rumtk_resolve_task!(f()).unwrap();
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_pipeline_quick_run_async {
        ( $pipeline:expr ) => {{
            use $crate::types::RUMBuffer;

            rumtk_pipeline_quick_run_async!($pipeline, &RUMBuffer::default())
        }};
        ( $pipeline:expr, $data:expr ) => {{
            use $crate::pipelines::pipeline_functions::{pipeline_await_pipeline};

            pipeline_await_pipeline($pipeline, $data)
        }};
    }

    ///
    /// Pipe a string buffer into pipeline.
    ///
    /// ## Example
    /// ```
    /// use rumtk_core::{rumtk_pipeline_pipe_string_data, rumtk_pipeline_command, rumtk_pipeline_quick_run};
    /// use rumtk_core::core::RUMResult;
    /// use rumtk_core::strings::buffer_to_string;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// const data: &str = "Hello World!";
    /// const expected: &str = "      0       2      12\n";
    ///
    ///
    /// let f = |input: &str| -> RUMResult<RUMBuffer> {
    ///     let mut pipeline = vec![
    ///         rumtk_pipeline_command!("wc")
    ///     ];
    ///
    ///     rumtk_pipeline_pipe_string_data!(&mut pipeline, input);
    ///
    ///     rumtk_pipeline_quick_run!(pipeline)
    /// };
    /// let result = buffer_to_string(&f(data).unwrap()).unwrap();
    ///
    /// assert_eq!(result, expected, "String correctly piped into pipeline!");
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_pipeline_pipe_string_data {
        ( $pipeline:expr, $data:expr ) => {{
            use $crate::{rumtk_pipeline_pipe_buffer};
            use $crate::strings::string_to_buffer;
            use $crate::pipelines::pipeline_functions::{pipeline_add_stdin_data_to_pipeline};

            let buffer = string_to_buffer($data);

            rumtk_pipeline_pipe_buffer!($pipeline, &buffer)
        }};
    }

    ///
    /// Pipe a [RUMBuffer] buffer into pipeline.
    ///
    /// ## Example
    /// ```
    /// use rumtk_core::{rumtk_pipeline_pipe_buffer, rumtk_pipeline_command, rumtk_pipeline_quick_run};
    /// use rumtk_core::core::RUMResult;
    /// use rumtk_core::strings::buffer_to_string;
    /// use rumtk_core::types::RUMBuffer;
    /// use rumtk_core::strings::string_to_buffer;
    ///
    /// const data: &str = "Hello World!";
    /// const expected: &str = "      0       2      12\n";
    ///
    ///
    /// let f = |input: &str| -> RUMResult<RUMBuffer> {
    ///     let mut pipeline = vec![
    ///         rumtk_pipeline_command!("wc")
    ///     ];
    ///
    ///     rumtk_pipeline_pipe_buffer!(&mut pipeline, &string_to_buffer(input));
    ///
    ///     rumtk_pipeline_quick_run!(pipeline)
    /// };
    /// let result = buffer_to_string(&f(data).unwrap()).unwrap();
    ///
    /// assert_eq!(result, expected, "String correctly piped into pipeline!");
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_pipeline_pipe_buffer {
        ( $pipeline:expr, $data:expr ) => {{
            use $crate::pipelines::pipeline_functions::{pipeline_add_stdin_data_to_pipeline};

            pipeline_add_stdin_data_to_pipeline($pipeline, $data)
        }};
    }

    ///
    /// Patch the pipeline's arguments with desired dynamic options.
    ///
    /// ## Example
    /// ```
    /// use rumtk_core::{rumtk_pipeline_patch_args, rumtk_pipeline_command, rumtk_pipeline_quick_run};
    /// use rumtk_core::core::RUMResult;
    /// use rumtk_core::strings::buffer_to_string;
    /// use rumtk_core::types::RUMBuffer;
    /// use rumtk_core::strings::string_to_buffer;
    ///
    ///
    /// let f = || -> RUMResult<RUMBuffer> {
    ///     let mut pipeline = vec![
    ///         rumtk_pipeline_command!("ls", RUMBuffer::default(), &vec![
    ///             "{options}".into()
    ///         ])
    ///     ];
    ///
    ///     rumtk_pipeline_patch_args!(&mut pipeline, &[("{options}", "-la")]);
    ///
    ///     rumtk_pipeline_quick_run!(pipeline)
    /// };
    ///
    /// let result_string = buffer_to_string(&f().unwrap()).unwrap();
    /// let results: Vec<&str> = result_string.as_str().split("\n").collect();
    /// let dot_dir = results.get(1).unwrap().chars().last().unwrap();
    ///
    /// assert_eq!(dot_dir, '.', "Incorrect options passed!");
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_pipeline_patch_args {
        ( $pipeline:expr, $replacements:expr ) => {{
            use $crate::pipelines::pipeline_functions::{pipeline_patch_args};

            pipeline_patch_args($pipeline, $replacements)
        }};
    }
}
