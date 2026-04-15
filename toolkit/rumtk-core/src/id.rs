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
use crate::types::RUMID;
use nanoid::nanoid;
use uuid::Uuid;

pub const DEFAULT_ID_SIZE: usize = 16;
pub enum RUMID_TYPE {
    SHORT,
    None,
}

pub fn generate_id(typ: RUMID_TYPE, size: usize) -> String {
    match typ {
        RUMID_TYPE::SHORT => {
            nanoid!(size)
        }
        RUMID_TYPE::None => RUMID::new_v4().to_string(),
    }
}

pub fn id_to_uuid(id: &str) -> RUMID {
    Uuid::parse_str(id).unwrap()
}

#[macro_export]
macro_rules! rumtk_generate_id {
    (  ) => {{
        use $crate::id::{generate_id, DEFAULT_ID_SIZE, RUMID_TYPE};

        generate_id(RUMID_TYPE::SHORT, DEFAULT_ID_SIZE)
    }};
    ( $size:expr ) => {{
        use $crate::id::{generate_id, DEFAULT_ID_SIZE, RUMID_TYPE};

        generate_id(RUMID_TYPE::SHORT, $size)
    }};
    ( $size:expr, $typ:expr ) => {{
        use $crate::id::{generate_id, DEFAULT_ID_SIZE, RUMID_TYPE};

        generate_id($typ, $size)
    }};
}
