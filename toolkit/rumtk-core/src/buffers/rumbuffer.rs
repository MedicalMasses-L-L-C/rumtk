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
use std::cmp::PartialEq;
use std::ops::Deref;
use std::ops::{Index, Range, RangeFull};
use std::sync::Arc;

type RUMBufferInner = Arc<RUMVec<u8>>;

///
/// The [RUMBuffer] type is meant to be a very lightweight owned buffer pointer. The impetus for building
/// this type is that benchmarking showed a potential vector getting allocated by the **bytes** crate.
/// I was using the bytes crate before because it is a very solid crate. However, the vector allocations worried me.
/// It was likely part of that crates interior mutability strategy with vtables but it was one of a few
/// areas consuming a lot of time and most importantly consuming kernel time by invoking malloc.
///
/// ## Example
/// ### Creation
/// ```
/// use rumtk_core::buffers::*;
///
/// let buffer = RUMBuffer::from("Hello World!");
/// let expected = b"Hello World!";
///
/// assert_eq!(&buffer[..], expected, "Could not create RUMBuffer!");
/// ```
///
/// ### Split Buffer
/// ```
/// use rumtk_core::buffers::*;
///
/// let buffer = RUMBuffer::from("Hello World!");
/// let section = buffer.split_to(5);
/// let expected = b"Hello";
///
/// assert_eq!(&section[..], expected, "Could not create RUMBuffer!");
/// ```
///
#[derive(Default, Debug, Clone)]
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

    #[inline]
    pub fn mutate(&mut self) -> RUMVec<u8> {
        self.data[self.offset..self.end].to_vec()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.end = self.offset + len;
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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

impl Deref for RUMBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data[self.offset..self.end]
    }
}

impl PartialEq for RUMBuffer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data[self.offset..self.end] == other.data[other.offset..other.end]
    }
}

///////////////////// Indexing ////////////////////////////////////

impl Index<usize> for RUMBuffer {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.offset + i]
    }
}

impl Index<Range<usize>> for RUMBuffer {
    type Output = [u8];
    fn index(&self, i: Range<usize>) -> &Self::Output {
        &self.data[self.offset + i.start.. self.offset + i.end]
    }
}

impl Index<RangeFull> for RUMBuffer {
    type Output = [u8];
    fn index(&self, i: RangeFull) -> &Self::Output {
        &self.data[self.offset.. self.end]
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

impl<const N: usize> From<&[u8; N]> for RUMBuffer {
    fn from(data: &[u8; N]) -> Self {
        Self::from(data.as_slice())
    }
}

impl From<&str> for RUMBuffer {
    fn from(data: &str) -> Self {
        Self::from(data.as_bytes())
    }
}