/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
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

pub mod cli_utils {
    use crate::core::{RUMResult, RUMVec};
    use crate::strings::{rumtk_format, EscapeExceptions, RUMString, RUMStringConversions};
    use crate::types::{RUMBuffer, RUMCLIParser};
    use compact_str::CompactStringExt;
    use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};
    use std::num::NonZeroU16;

    pub const BUFFER_SIZE: usize = 1024 * 4;
    pub const BUFFER_CHUNK_SIZE: usize = 512;
    const END_OF_MESSAGE: &[u8; 8] = b"\0\0\0\0\0\0\0\0";

    pub static CLI_ESCAPE_EXCEPTIONS: EscapeExceptions =
        &[("\\n", "\n"), ("\\r", "\r"), ("\\\\", "\\")];

    pub type BufferSlice = Vec<u8>;
    pub type BufferChunk = [u8; BUFFER_CHUNK_SIZE];

    ///
    /// Example CLI parser that can be used to paste in your binary and adjust as needed.
    ///
    /// Note, this is only an example.
    ///
    #[derive(RUMCLIParser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct RUMTKArgs {
        ///
        /// For interface crate only. Specifies the ip address to connect to.
        ///
        /// In outbound mode, `--ip` and `--port` are required parameters.
        ///
        /// In inbound mode, you can omit either or both parameters.
        ///
        #[arg(short, long)]
        ip: Option<RUMString>,
        ///
        /// For interface crate only. Specifies the port to connect to.
        ///
        /// In outbound mode, `--ip` and `--port` are required parameters.
        ///
        /// In inbound mode, you can omit either or both parameters.
        ///
        #[arg(short, long)]
        port: Option<NonZeroU16>,
        ///
        /// For process crate only. Specifies command line script to execute on message.
        ///
        #[arg(short, long)]
        x: Option<RUMString>,
        ///
        /// Number of processing threads to allocate for this program.
        ///
        #[arg(short, long, default_value_t = 1)]
        threads: usize,
        ///
        /// For interface crate only. Specifies if the interface is in outbound mode.
        ///
        /// In outbound mode, `--ip` and `--port` are required parameters.
        ///
        /// In inbound mode, you can omit either or both parameters.
        ///
        #[arg(short, long)]
        outbound: bool,
        ///
        /// Request program runs in debug mode and log more information.
        ///
        #[arg(short, long, default_value_t = false)]
        debug: bool,
        ///
        /// Request program runs in dry run mode and simulate as many steps as possible but not commit
        /// to a critical non-reversible step.
        ///
        /// For example, if it was meant to write contents to a file, stop before doing so.
        ///
        #[arg(short, long, default_value_t = false)]
        dry_run: bool,
    }

    ///
    /// Consumes the incoming buffer in chunks of [BUFFER_CHUNK_SIZE](BUFFER_CHUNK_SIZE) bytes size
    /// until no more bytes are present.
    ///
    /// ## Example
    ///
    /// ```
    /// use rumtk_core::cli::cli_utils::{read_stdin};
    ///
    /// let stdin_data = read_stdin().unwrap();
    ///
    /// assert_eq!(stdin_data.len(), 0, "Returned data with {} size even though we expected 0 bytes!", stdin_data.len())
    /// ```
    ///
    pub fn read_stdin() -> RUMResult<RUMBuffer> {
        let mut stdin_handle = stdin();
        let mut stdin_buffer = RUMVec::with_capacity(BUFFER_SIZE);
        let mut s = read_some_stdin(&mut stdin_handle, &mut stdin_buffer)?;

        while s > 0 {
            s = read_some_stdin(&mut stdin_handle, &mut stdin_buffer)?;
        }

        Ok(RUMBuffer::from(stdin_buffer))
    }

    ///
    /// Consumes the incoming buffer in chunks of [BUFFER_CHUNK_SIZE](BUFFER_CHUNK_SIZE) bytes size.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::io::stdin;
    /// use std::io::prelude::*;
    /// use std::process::{Command, Stdio};
    /// use rumtk_core::cli::cli_utils::{read_some_stdin, BUFFER_SIZE, BUFFER_CHUNK_SIZE};
    ///
    /// let mut stdin_lock = stdin().lock();
    /// let mut stdin_buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
    /// let mut s = read_some_stdin(&mut stdin_lock, &mut stdin_buffer).unwrap();
    /// let mut totas_s = s;
    /// while s > 0 {
    ///    s = read_some_stdin(&mut stdin_lock, &mut stdin_buffer).unwrap();
    ///    totas_s += s;
    /// }
    ///
    /// assert_eq!(totas_s, 0, "Returned data with {} size even though we expected 0 bytes!", totas_s)
    /// ```
    ///
    pub fn read_some_stdin(input: &mut Stdin, buf: &mut BufferSlice) -> RUMResult<usize> {
        let mut chunk: BufferChunk = [0; BUFFER_CHUNK_SIZE];
        match input.read(&mut chunk) {
            Ok(s) => {
                let slice = &chunk[0..s];

                if s == END_OF_MESSAGE.len() && slice == END_OF_MESSAGE {
                    return Ok(0);
                }

                if s > 0 {
                    buf.extend_from_slice(slice);
                }

                Ok(s)
            }
            Err(e) => Err(rumtk_format!("Error reading stdin chunk because {}!", e)),
        }
    }

    ///
    /// writes [`stringview`] to `stdout`.
    ///
    pub fn write_string_stdout(data: &str) -> RUMResult<()> {
        write_stdout(&data.to_buffer())
    }

    ///
    /// Writes [RUMBuffer] to `stdout`.
    ///
    pub fn write_stdout(data: &RUMBuffer) -> RUMResult<()> {
        // Grab handle
        let mut stdout_handle = stdout();
        // Write the buffer out
        write_buffer(&mut stdout_handle, data)?;
        write_buffer(&mut stdout_handle, END_OF_MESSAGE)
    }

    fn write_buffer(stdout: &mut Stdout, buffer: &[u8]) -> RUMResult<()> {
        match stdout.write_all(buffer) {
            Ok(_) => Ok(flush_buffer(stdout)?),
            Err(e) => Err(rumtk_format!("Error writing to stdout because => {}", e)),
        }
    }

    fn flush_buffer(stdout: &mut Stdout) -> RUMResult<()> {
        match stdout.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(rumtk_format!("Error flushing stdout because => {}", e)),
        }
    }

    pub fn print_license_notice(program: &str, year: &str, author_list: &Vec<&str>) {
        let authors = author_list.join_compact(", ");
        let notice = rumtk_format!(
            "  {program}  Copyright (C) {year}  {authors}
        This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
        This is free software, and you are welcome to redistribute it
        under certain conditions; type `show c' for details."
        );
        println!("{}", notice);
    }
}

pub mod macros {
    ///
    /// Reads STDIN and unescapes the incoming message.
    /// Return this unescaped message.
    ///
    /// # Example
    /// ```
    /// use rumtk_core::core::RUMResult;
    /// use rumtk_core::types::RUMBuffer;
    /// use rumtk_core::rumtk_read_stdin;
    ///
    /// fn test_read_stdin() -> RUMResult<RUMBuffer> {
    ///     rumtk_read_stdin!()
    /// }
    ///
    /// match test_read_stdin() {
    ///     Ok(s) => (),
    ///     Err(e) => panic!("Error reading stdin because => {}", e)
    /// }
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_read_stdin {
        (  ) => {{
            use $crate::cli::cli_utils::read_stdin;
            read_stdin()
        }};
    }

    ///
    /// Writes [RUMString](crate::strings::RUMString) or [RUMBuffer](crate::types::RUMBuffer) to `stdout`.
    ///
    /// If the `binary` parameter is omitted, we take a [RUMString](crate::strings::RUMString), escape it
    /// while preserving [CLI_ESCAPE_EXCEPTIONS](crate::cli::cli_utils::CLI_ESCAPE_EXCEPTIONS) characters,
    /// and finally write it out as a [RUMBuffer](crate::types::RUMBuffer) to `stdout`.
    ///
    /// If the `binary` parameter is passed, we push the `message` parameter directly to `stdout`. the
    /// `message` parameter has to be of type [RUMBuffer](crate::types::RUMBuffer).
    ///
    /// ## Example
    ///
    /// ### Default / Pushing a String
    /// ```
    /// use rumtk_core::rumtk_write_stdout;
    ///
    /// rumtk_write_stdout!("I ❤ my wife!");
    /// ```
    ///
    /// ## Pushing Binary Buffer
    /// ```
    /// use rumtk_core::rumtk_write_stdout;
    /// use rumtk_core::core::new_random_buffer;
    ///
    /// let buffer = new_random_buffer();
    /// rumtk_write_stdout!(buffer, true);
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_write_stdout {
        ( $message:expr ) => {{
            use $crate::cli::cli_utils::write_string_stdout;
            write_string_stdout(&$message)
        }};
        ( $message:expr, $binary:expr ) => {{
            use $crate::cli::cli_utils::write_stdout;
            write_stdout(&$message)
        }};
    }

    ///
    /// Prints the mandatory GPL License Notice to terminal!
    ///
    /// # Example
    /// ## Default
    /// ```
    /// use rumtk_core::rumtk_print_license_notice;
    ///
    /// rumtk_print_license_notice!();
    /// ```
    /// ## Program Only
    /// ```
    /// use rumtk_core::rumtk_print_license_notice;
    ///
    /// rumtk_print_license_notice!("RUMTK");
    /// ```
    /// ## Program + Year
    /// ```
    /// use rumtk_core::rumtk_print_license_notice;
    ///
    /// rumtk_print_license_notice!("RUMTK", "2025");
    /// ```
    /// ## Program + Year + Authors
    /// ```
    /// use rumtk_core::rumtk_print_license_notice;
    ///
    /// rumtk_print_license_notice!("RUMTK", "2025", &vec!["Luis M. Santos, M.D."]);
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_print_license_notice {
        ( ) => {{
            use $crate::cli::cli_utils::print_license_notice;

            print_license_notice("RUMTK", "2025", &vec!["Luis M. Santos, M.D."]);
        }};
        ( $program:expr ) => {{
            use $crate::cli::cli_utils::print_license_notice;
            print_license_notice(&$program, "2025", &vec!["2025", "Luis M. Santos, M.D."]);
        }};
        ( $program:expr, $year:expr ) => {{
            use $crate::cli::cli_utils::print_license_notice;
            print_license_notice(&$program, &$year, &vec!["Luis M. Santos, M.D."]);
        }};
        ( $program:expr, $year:expr, $authors:expr ) => {{
            use $crate::cli::cli_utils::print_license_notice;
            print_license_notice(&$program, &$year, &$authors);
        }};
    }
}
