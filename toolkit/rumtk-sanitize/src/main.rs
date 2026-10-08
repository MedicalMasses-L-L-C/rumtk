/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Ian McDonald
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

use rumtk_core::{rumtk_deserialize, rumtk_read_stdin, rumtk_serialize, rumtk_write_stdout};
use rumtk_core::base::{RUMResult, RUMVec};
use rumtk_core::strings::{RUMArrayConversions, RUMString};
use rumtk_core::types::RUMCLIParser;
use rumtk_web::{utils::sanitize_html};

///
/// # CLI Sanitize Utility
///
/// ## Usage:
///
///
///     cat BUILD_injectionDemo.html | .\target\debug\rumtk-sanitize --mode html
///
///
///  TODO:  Revisit macro's on September 25th (if it is not the 25th still do this)
///  TODO:  Rebuild the interface args
///

#[derive(RUMCLIParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RUMTKInterfaceArgs {
    ///
    /// Specifies command line script to execute on message.
    ///
    #[arg(short, long)]
    mode: Option<String>,
}

fn process_message(args: &RUMTKInterfaceArgs) -> RUMResult<()> {
    let stdin_msg = rumtk_read_stdin!()?;
    if !stdin_msg.is_empty() {
        let out_data = match &args.mode {
            Some(mode) => {
                match mode.to_lowercase().as_str() {
                    "html" => sanitize_html(&stdin_msg.as_slice().to_string()?, false),
                    _ => return Err("Invalid mode".into()),
                }
            },
            None => sanitize_html(&stdin_msg.as_slice().to_string()?, false),
        };

        rumtk_write_stdout!(&out_data);
    }
    Ok(())
}

fn process_message_loop(args: &RUMTKInterfaceArgs) {
    loop {
        match process_message(args) {
            Ok(()) => continue,
            Err(e) => println!("{}", e), // TODO: missing log call
        };
    }
}

fn main() {
    let args = RUMTKInterfaceArgs::parse();
    process_message_loop(&args);
}
