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

use rumtk_core::base::RUMResult;
use rumtk_core::strings::{RUMArrayConversions, RUMString};
use rumtk_core::types::RUMCLIParser;
use rumtk_core::{rumtk_cli_process, rumtk_serialize};
use rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message;
use rumtk_hl7_v2::{rumtk_v2_generate_message, rumtk_v2_parse_message};

/*
///
/// HL7 V2 Parser CLI
///
rumtk_cli_define_args!(
    ///
    /// Specifies whether to output the pretty print version of the message. This option has no
    /// effect for raw v2 messages piped into the program
    ///
    #[arg(short, long)]
    pretty: bool
);
*/

fn main() {
    rumtk_cli_process!(
        ///
        /// Specifies whether to output the pretty print version of the message. This option has no
        /// effect for raw v2 messages piped into the program
        ///
        #[arg(short, long)]
        pretty: bool,
        |msg, args: &RUMTKInterfaceArgs| -> RUMResult<RUMString> {
            let parsed_msg: V2Message = msg;
            Ok(rumtk_v2_generate_message!(&parsed_msg))
        },
        |data, args: &RUMTKInterfaceArgs| -> RUMResult<RUMString> {
            let msg = rumtk_v2_parse_message!(data)?;

            if !args.quiet {
                Ok(rumtk_serialize!(&msg)?)
                //RUMString::default()
            } else {
                Ok(RUMString::default())
            }
        }
    );
}
