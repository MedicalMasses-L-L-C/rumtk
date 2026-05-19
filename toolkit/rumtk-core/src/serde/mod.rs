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
use crate::buffers::buffer_to_str;
pub use crate::types::RUMOrderedMap;
use crate::types::{RUMBuffer, RUMBufferMut};
use core::str::Chars;
pub use json::*;
pub use nanoserde::{DeJsonErr, DeJsonState, DeJsonTok, SerJsonState};
use std::hash::Hash;

pub mod json;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RUMSerializableOrderedMap<K: Hash + Eq, V>(pub RUMOrderedMap<K, V>);

impl<K, V> RUMSerJson for RUMSerializableOrderedMap<K, V>
where
    K: RUMSerJson + Hash + Eq,
    V: RUMSerJson,
{
    fn ser_json(&self, d: usize, s: &mut SerJsonState) {
        s.out.push('{');
        let len = self.0.len();
        let indent = d + 1;
        for (index, (k, v)) in self.0.iter().enumerate() {
            s.indent(indent);
            k.ser_json(indent, s);
            s.out.push(':');
            v.ser_json(indent, s);
            if (index + 1) < len {
                s.conl();
            }
        }
        s.indent(d);
        s.out.push('}');
    }
}

impl<K, V> RUMDeJson for RUMSerializableOrderedMap<K, V>
where
    K: RUMDeJson + Eq + core::hash::Hash,
    V: RUMDeJson,
{
    fn de_json(s: &mut DeJsonState, i: &mut Chars) -> Result<RUMSerializableOrderedMap<K, V>, DeJsonErr> {
        let mut h = RUMOrderedMap::new();

        s.curly_open(i)?;
        while s.tok != DeJsonTok::CurlyClose {
            let k = RUMDeJson::de_json(s, i)?;
            s.colon(i)?;
            let v = RUMDeJson::de_json(s, i)?;
            s.eat_comma_curly(i)?;
            h.insert(k, v);
        }
        s.curly_close(i)?;

        Ok(RUMSerializableOrderedMap(h))
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RUMSerializableBuffer(pub RUMBuffer);

impl RUMSerJson for RUMSerializableBuffer
{
    fn ser_json(&self, d: usize, s: &mut SerJsonState) {
        let escaped = buffer_to_str(&self.0).unwrap_or_else(|e| {
            eprintln!("{}", e);
            ""
        });
        s.out.push('"');
        s.out.push_str(escaped);
        s.out.push('"');
    }
}

impl RUMDeJson for RUMSerializableBuffer
{
    fn de_json(s: &mut DeJsonState, i: &mut Chars) -> Result<RUMSerializableBuffer, DeJsonErr> {
        let mut h = RUMBufferMut::new();

        s.curly_open(i)?;
        while s.tok != DeJsonTok::CurlyClose {
            println!("{:?}", &s.tok);
            s.eat_comma_curly(i)?;
        }
        s.curly_close(i)?;

        Ok(RUMSerializableBuffer(h.freeze()))
    }
}