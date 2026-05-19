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
pub use crate::buffers::{RUMBuffer, RUMBufferMut};
pub use clap::Parser as RUMCLIParser;
pub use indexmap::IndexMap as RUMOrderedMap;
use std::any::TypeId;
pub use std::collections::HashMap as RUMHashMap;
pub use tokio::net::TcpListener as RUMTcpListener;
pub use uuid::Uuid as RUMID;

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
