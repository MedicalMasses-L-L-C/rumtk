/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
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

pub mod python {
    use crate::hl7_v2_parser::v2_parser::V2Message;
    use rumtk_core::core::RUMResult;
    use rumtk_core::scripting::python_utils::RUMPython;
    use rumtk_core::scripting::python_utils::{py_extract_any, py_new_args, py_push_arg};
    use rumtk_core::strings::RUMString;
    use rumtk_core::{rumtk_python_exec, rumtk_python_exec_module};

    const EXPECTED_PROCESSOR_FUNCTION_NAME: &str = "process";

    ///
    /// Takes a [V2Message] and pass it to a Python module for processing. After processing, we expect to
    /// receive a [V2Message] result with the modified copy of the message.
    ///
    ///
    pub fn process_message(module_path: &RUMString, message: &V2Message) -> RUMResult<V2Message> {
        let closure = |py: RUMPython| -> RUMResult<V2Message> {
            let mut args = py_new_args(py);
            py_push_arg(py, &mut args, message)?;

            let result = rumtk_python_exec_module!(
                py,
                &module_path,
                EXPECTED_PROCESSOR_FUNCTION_NAME,
                &args
            );
            let val: V2Message = py_extract_any(py, &result)?;
            Ok(val)
        };

        rumtk_python_exec!(closure)
    }
}

pub mod python_macros {
    ///
    /// Macro for processing V2 message via a Python module loaded from disk.
    ///This interface attempts to cache the module to avoid repeated loads of the module.
    ///
    /// ## Examples
    ///
    /// ```
    ///     use rumtk_hl7_v2::{rumtk_v2_parse_message};
    ///     let pattern = "MSH1.1";
    ///     let hl7_v2_message = "MSH|^~\\&|NISTEHRAPP|NISTEHRFAC|NISTIISAPP|NISTIISFAC|20150625072816.601-0500||VXU^V04^VXU_V04|NIST-IZ-AD-10.1_Send_V04_Z22|P|2.5.1|||ER|AL|||||Z22^CDCPHINVS|NISTEHRFAC|NISTIISFAC\n";
    ///     let message = rumtk_v2_parse_message!(&hl7_v2_message).unwrap();
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_v2_python_exec {
        ( $mod_path:expr, $message:expr ) => {{
            use $crate::hl7_v2_scripting::python::process_message;

            process_message($mod_path, $message)
        }};
    }
}
