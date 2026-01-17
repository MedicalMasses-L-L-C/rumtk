/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2025  Luis M. Santos, M.D.
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

use rumtk_core::core::RUMResult;
use rumtk_core::strings::RUMString;
use rumtk_core::types::RUMCLIParser;
use rumtk_core::{rumtk_deserialize, rumtk_read_stdin, rumtk_serialize, rumtk_write_stdout};
use rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message;
use rumtk_hl7_v2::{rumtk_v2_generate_message, rumtk_v2_parse_message};

///
/// HL7 V2 Parser CLI
///
#[derive(RUMCLIParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RUMTKInterfaceArgs {
    ///
    /// Specifies command line script to execute on message.
    ///
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
    ///
    /// Specifies whether to output the pretty print version of the message. This option has no
    /// effect for raw v2 messages piped into the program
    ///
    #[arg(short, long)]
    pretty: bool,
    ///
    /// Only used if in client/outbound mode. Places the interface into a loop constantly looking
    /// for messages in stdin to ship to the connected listening interface.
    ///
    #[arg(short, long)]
    daemon: bool,
}

fn process_message(pretty_print: bool) -> RUMResult<()> {
    let stdin_msg = rumtk_read_stdin!()?;
    if !stdin_msg.is_empty() {
        let out_data = match rumtk_deserialize!(&stdin_msg) {
            Ok(msg) => {
                let parsed_msg: V2Message = msg;
                rumtk_v2_generate_message!(&parsed_msg)
            }
            Err(e) => {
                let msg = rumtk_v2_parse_message!(&stdin_msg)?;
                match rumtk_serialize!(&msg, &pretty_print)?.parse() {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(RUMString::from(
                            "Failure to identify and process message in stdin. It might not be a valid V2Message or v2 raw message!",
                        ));
                    }
                }
            }
        };
        rumtk_write_stdout!(&out_data);
    }
    Ok(())
}

fn process_message_loop(pretty_print: bool) {
    loop {
        match process_message(pretty_print) {
            Ok(()) => continue,
            Err(e) => println!("{}", e), // TODO: missing log call
        };
    }
}

fn main() {
    let args = RUMTKInterfaceArgs::parse();

    if args.daemon {
        process_message_loop(args.pretty);
    } else {
        process_message(args.pretty).expect("Failed to generate V2 message");
    }
}
