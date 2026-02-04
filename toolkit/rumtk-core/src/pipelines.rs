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
    use crate::strings::{RUMString, RUMStringConversions};
    use crate::types::{RUMBuffer, RUMHashMap};

    use crate::core::{RUMResult, RUMVec};
    use std::process::{Child, Command};

    pub type RUMCommandArgs = Vec<RUMString>;
    pub type RUMCommandEnv = RUMHashMap<RUMString, RUMString>;
    #[derive(Default, Debug, Clone)]
    pub struct RUMCommand {
        pub path: RUMString,
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
    use crate::strings::rumtk_format;
    use std::io::{Read, Write};

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

    ///
    /// Closes the `stdin` standard in file for process. Useful to trigger a resolution of the pipeline.
    ///
    /// ## Example
    ///
    /// ```
    /// use rumtk_core::pipelines::pipeline_functions::pipeline_close_process_stdin;
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::pipelines::pipeline_types::{RUMCommand, RUMPipelineCommand};
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_command, pipeline_pipe_into_process, pipeline_spawn_process};
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_generate_command(&ls_command);
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
    /// use rumtk_core::pipelines::pipeline_functions::{pipeline_generate_command, pipeline_pipe_into_process, pipeline_spawn_process};
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let ls_name = "ls";
    /// let mut ls_command = RUMCommand::default();
    /// ls_command.path = RUMString::from(ls_name);
    /// let mut sys_ls_command = pipeline_generate_command(&ls_command);
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
        data: &Option<RUMBuffer>,
    ) -> RUMResult<()> {
        match data {
            Some(data) => match process.stdin {
                Some(ref mut stdin) => match stdin.write_all(&data) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(rumtk_format!(
                            "Failed to pipe data to stdin of process because => {}",
                            e
                        ))
                    }
                },
                None => {}
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
    pub fn pipeline_generate_pipeline(commands: &RUMCommandLine) -> RUMResult<RUMPipeline> {
        let first_command = commands.first().unwrap();

        // Setup pipeline
        let mut pipeline = vec![];

        //Bootstrap first process in chain
        let mut root = pipeline_generate_command(&first_command);
        let mut parent_process = pipeline_spawn_process(&mut root)?;
        pipeline_pipe_into_process(&mut parent_process, &mut first_command.data.clone())?;
        pipeline.push(parent_process);

        for cmd in commands.iter().skip(1) {
            let mut new_root = pipeline_generate_command(cmd);
            pipeline_pipe_process(pipeline.last_mut().unwrap(), &mut new_root)?;
            parent_process = pipeline_spawn_process(&mut new_root)?;
            pipeline.push(parent_process);
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
    /// let result = rumtk_resolve_task!(pipeline_await_pipeline(pipeline)).unwrap().unwrap();
    ///
    /// assert_eq!(result.is_empty(), false, "Pipeline returned no buffer from command wc! => {:?}", &result);
    /// ```
    ///
    pub async fn pipeline_await_pipeline(mut pipeline: RUMPipeline) -> RUMPipelineResult {
        // Let's make sure the stdin is closed on the first process to make sure it exits instead of
        // remain waiting for EOF in the stdin stream.
        pipeline_close_process_stdin(pipeline.first_mut().unwrap());

        // Now let's visit each process and await their completion!
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
        // Let's make sure the stdin is closed on the first process to make sure it exits instead of
        // remain waiting for EOF in the stdin stream.
        pipeline_close_process_stdin(pipeline.first_mut().unwrap());

        // Now let's visit each process and await their completion!
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
    /// use rumtk_core::core::{RUMResult, new_random_buffer};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = || -> RUMResult<()> {
    ///     let result = rumtk_pipeline_run!(
    ///         rumtk_pipeline_command!("ls", new_random_buffer()),
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

            let pipeline = pipeline_generate_pipeline(&vec![
                $($command),+
            ])?;

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
    /// use rumtk_core::core::{RUMResult, new_random_buffer};
    /// use rumtk_core::strings::RUMStringConversions;
    /// use rumtk_core::types::RUMBuffer;
    ///
    /// let f = async || -> RUMResult<()> {
    ///     let result = rumtk_pipeline_run_async!(
    ///         rumtk_pipeline_command!("ls", new_random_buffer()),
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
    #[macro_export]
    macro_rules! rumtk_pipeline_run_async {
        ( $($command:expr),+ ) => {{
            use $crate::pipelines::pipeline_functions::{pipeline_generate_pipeline, pipeline_await_pipeline};

            let pipeline = pipeline_generate_pipeline(&vec![
                $($command),+
            ])?;

            pipeline_await_pipeline(pipeline)
        }};
    }
}
