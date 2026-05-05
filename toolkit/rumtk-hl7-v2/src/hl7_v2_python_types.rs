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
use crate::hl7_v2_base_types::v2_base_types::V2Result;
use crate::hl7_v2_parser::v2_parser::V2Message;
use pyo3::pyclass;
use rumtk_core::strings::RUMString;
use rumtk_core::types::RUMBuffer;
use std::sync::Arc;

#[derive(Default, Debug, PartialEq, Clone)]
#[pyclass]
pub struct PyV2Message {
    data: Arc<V2Message>,
}

impl TryFrom<&V2Message> for PyV2Message {
    type Error = RUMString;
    fn try_from(msg: &V2Message) -> V2Result<PyV2Message> {
        Ok(PyV2Message {
            data: Arc::new(msg.clone()),
        })
    }
}

impl TryFrom<V2Message> for PyV2Message {
    type Error = RUMString;
    fn try_from(msg: V2Message) -> V2Result<PyV2Message> {
        Ok(PyV2Message {
            data: Arc::new(msg),
        })
    }
}