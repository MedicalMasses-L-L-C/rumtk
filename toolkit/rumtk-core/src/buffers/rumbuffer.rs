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
use crate::mem::{as_slice, as_slice_mut, copy_from_slice, AsSlice};
use rumtk_arena::dune::Dune;
use rumtk_arena::rumtk_dune_new;
use std::cmp::PartialEq;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::{Index, Range, RangeFull};
use std::sync::LazyLock;

pub type RUMBufferInner = Option<Dune>;

static EMPTY_BUFFER_DATA: [u8;1] = [0;1];
static EMPTY_RUMBUFFER: LazyLock<RUMBuffer> = LazyLock::new(|| RUMBuffer::new_static());

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
    ptr: *const u8,
    size: usize,
}

impl RUMBuffer {
    #[inline]
    pub fn new_static() -> Self {
        let data_length = 0;
        let ptr = EMPTY_BUFFER_DATA.as_ptr();
        Self {
            data: None,
            ptr,
            size: data_length,
        }
    }
    #[inline]
    pub fn new() -> Self {
        EMPTY_RUMBUFFER.clone()
    }

    #[inline]
    fn from_slice(data: &[u8]) -> Self {
        let data_length = data.len();
        let mut mem = rumtk_dune_new!(data_length);
        let mut ptr = mem.allocate_raw(data_length).unwrap();
        let dst = as_slice_mut(ptr, data_length);
        copy_from_slice(&data[..], dst);
        Self {
            data: Some(mem),
            ptr,
            size: data_length,
        }
    }

    #[inline]
    pub fn split_to(&mut self, offset: usize) -> Self {
        assert!(offset <= self.size, "offset too large");
        let copy = Self {
            data: None,
            ptr: self.ptr.clone(),
            size: offset,
        };
        self.ptr = unsafe { self.ptr.add(offset) };
        self.size -= offset;
        copy
    }

    #[inline]
    pub fn freeze(&self) -> Self {
        Self {
            data: None,
            ptr: self.ptr.clone(),
            size: self.size.clone(),
        }
    }

    #[inline]
    pub fn mutate(&mut self) -> Self {
        self.freeze()
    }

    #[inline]
    pub fn to_vec(&mut self) -> RUMVec<u8> {
        self.as_slice().to_vec()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.size = len;
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn is_buffer(&self) -> bool {
        self.data.is_some()
    }

    #[inline]
    pub fn is_view(&self) -> bool {
        self.data.is_none()
    }
}

impl AsSlice for RUMBuffer {
    type Item = u8;

    #[inline]
    fn as_slice(&self) -> &[Self::Item] {
        as_slice(self.ptr,  self.size)
    }

    #[inline]
    fn as_slice_mut(&self) -> &mut [Self::Item] {
        as_slice_mut(self.ptr as *mut u8,  self.size)
    }
}

impl AsRef<[u8]> for RUMBuffer {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Deref for RUMBuffer {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for RUMBuffer {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_slice_mut()
    }
}

impl PartialEq for RUMBuffer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
/*
impl Drop for RUMBuffer {
    fn drop(&mut self) {
        match self.data.clone() {
            Some(mem) => {
                drop(mem)
            },
            None => {}
        }
    }
}
*/
unsafe impl Send for RUMBuffer {}
unsafe impl Sync for RUMBuffer {}

///////////////////// Indexing ////////////////////////////////////

impl Index<usize> for RUMBuffer {
    type Output = u8;
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.as_slice()[i]
    }
}

impl Index<Range<usize>> for RUMBuffer {
    type Output = [u8];
    #[inline]
    fn index(&self, i: Range<usize>) -> &Self::Output {
        &self.as_slice()[i.start..i.end]
    }
}

impl Index<RangeFull> for RUMBuffer {
    type Output = [u8];
    #[inline]
    fn index(&self, i: RangeFull) -> &Self::Output {
        self.as_slice()
    }
}

/////////////////////// Conversions ////////////////////////////////
impl From<RUMVec<u8>> for RUMBuffer {
    #[inline]
    fn from(data: RUMVec<u8>) -> Self {
        Self::from_slice(data.as_slice())
    }
}

impl From<&[u8]> for RUMBuffer {
    #[inline]
    fn from(data: &[u8]) -> Self {
        Self::from_slice(data)
    }
}

impl<const N: usize> From<&[u8; N]> for RUMBuffer {
    #[inline]
    fn from(data: &[u8; N]) -> Self {
        Self::from(data.as_slice())
    }
}

impl From<&str> for RUMBuffer {
    #[inline]
    fn from(data: &str) -> Self {
        Self::from(data.as_bytes())
    }
}