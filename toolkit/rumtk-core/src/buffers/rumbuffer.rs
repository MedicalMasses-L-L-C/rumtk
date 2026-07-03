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
use rumtk_arena::rumtk_dune_nullptr;
use std::sync::{Arc, RwLock};

type RUMBufferInner = Arc<RwLock<Vec<u8>>>;


#[derive(Debug, PartialEq)]
pub struct RUMBuffer {
    data: RUMBufferInner,
    drop: bool,
}

impl RUMBuffer {
    pub const fn new() -> Self {
        Self {
            data: rumtk_dune_nullptr!(),
            drop: false,
        }
    }

    pub fn split_to(&self, index: usize) -> Self {
        Self {
            data: &self.data[..index],
            drop: false
        }
    }
}

trait AsSlice {
    type Item;
    fn as_slice(&self) -> &[Self::Item];
}

impl AsSlice for RUMBuffer {
    type Item = u8;

    fn as_slice(&self) -> &[Self::Item] {
        self.data
    }
}

impl AsRef<[u8]> for RUMBuffer {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}