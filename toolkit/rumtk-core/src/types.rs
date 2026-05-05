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
use crate::strings::{buffer_to_string, string_to_buffer};
pub use ahash::AHashMap as RUMHashMap;
pub use bytes::Bytes as RUMBuffer;
pub use clap::Parser as RUMCLIParser;
pub use indexmap::IndexMap as RUMOrderedMap;
pub use serde;
pub use serde::{
    Deserialize as RUMDeserialize, Deserializer as RUMDeserializer, Serialize as RUMSerialize,
    Serializer as RUMSerializer,
};
use std::any::TypeId;
pub use tokio::net::TcpListener as RUMTcpListener;
pub use uuid::Uuid as RUMID;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SerdeRUMBufferProxy {
    pub inner: RUMBuffer,
}

impl RUMSerialize for SerdeRUMBufferProxy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Convert external type to a serializable format
        let escaped = match buffer_to_string(&self.inner) {
            Ok(string) => string,
            Err(err) => return Err(serde::ser::Error::custom(err)),
        };
        serializer.serialize_str(escaped.as_str())
    }
}

impl<'de> RUMDeserialize<'de> for SerdeRUMBufferProxy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let escaped_val = String::deserialize(deserializer)?;
        let val = escaped_val;
        Ok(SerdeRUMBufferProxy{
            inner: string_to_buffer(&val)
        })
    }
}

///
/// Helper for quickly checking if incoming data is of an expected type.
/// 
/// ## Example
/// ```
/// use rumtk_core::types::is_type;
/// 
/// let i = 5;
/// let j = 10;
/// let is_int = is_type(&i, &j);
/// 
/// assert!(is_int, "The compared type is not an integer!")
/// ```
/// 
pub const fn is_type<T: 'static, R: 'static>(_: &T, target: &R) -> bool {
    TypeId::of::<T>() == TypeId::of::<R>()
}
