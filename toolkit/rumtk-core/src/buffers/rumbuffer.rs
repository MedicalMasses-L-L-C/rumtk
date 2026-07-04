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
use crate::base::RUMVec;
use std::sync::Arc;

type RUMBufferInner = Arc<RUMVec<u8>>;


#[derive(Default, Debug, PartialEq, Clone)]
pub struct RUMBuffer {
    data: RUMBufferInner,
    offset: usize,
    end: usize,
}

impl RUMBuffer {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Vec::new()),
            offset: 0,
            end: 0,
        }
    }

    pub fn split_to(&self, offset: usize) -> Self {
        Self {
            data: self.data.clone(),
            offset: self.end,
            end: self.offset + offset,
        }
    }
}

pub trait AsSlice {
    type Item;
    fn as_slice(&self) -> &[Self::Item];
}

impl AsSlice for RUMBuffer {
    type Item = u8;

    fn as_slice(&self) -> &[Self::Item] {
        &self.data[self.offset..self.end]
    }
}

impl AsRef<[u8]> for RUMBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.data[self.offset..self.end]
    }
}


/////////////////////// Conversions ////////////////////////////////
impl From<RUMVec<u8>> for RUMBuffer {
    fn from(data: RUMVec<u8>) -> Self {
        let data_length = data.len();
        Self {
            data: Arc::new(data),
            offset: 0,
            end: data_length,
        }
    }
}

impl From<&[u8]> for RUMBuffer {
    fn from(data: &[u8]) -> Self {
        let owned_data: Vec<u8> = data.to_vec();
        let data_length = owned_data.len();
        Self {
            data: Arc::new(owned_data),
            offset: 0,
            end: data_length,
        }
    }
}

impl From<&str> for RUMBuffer {
    fn from(data: &str) -> Self {
        Self::from(data.as_bytes())
    }
}