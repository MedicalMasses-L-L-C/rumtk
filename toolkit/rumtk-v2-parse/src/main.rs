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
#![feature(str_as_str)]

use rumtk_core::base::{RUMResult, RUMVec};
use rumtk_core::cli::cli_utils::CliProcessor;
use rumtk_core::strings::RUMArrayConversions;
use rumtk_core::strings::RUMString;
use rumtk_core::types::RUMCLIParser;
use rumtk_core::{rumtk_cli_process, rumtk_deserialize, rumtk_serialize};
use rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message;
use rumtk_hl7_v2::{rumtk_v2_generate_message, rumtk_v2_parse_message};

fn main() {
    rumtk_cli_process!(
        ///
        /// Specifies whether to output the pretty print version of the message. This option has no
        /// effect for raw v2 messages piped into the program
        ///
        #[arg(short, long)]
        pretty: bool,
        CliProcessor::Message(|data: RUMVec<u8>, args: &RUMTKInterfaceArgs| -> RUMResult<RUMString> {
            match rumtk_deserialize!(&data.to_string()?) {
                Ok(msg) => {
                    let parsed_msg: V2Message = msg;
                    Ok(rumtk_v2_generate_message!(&parsed_msg))
                }
                Err(_) => {
                    let msg = rumtk_v2_parse_message!(data)?;

                    let r = match args.quiet {
                        true => RUMString::default(),
                        false => rumtk_serialize!(&msg)?
                    };
                    Ok(r)
                }
            }
        })
    );
}
