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
use crate::cpu::CPU_SIMD_64_SIZE;
use crate::strings::RUMString;
/////////////////If using MiMalloc//////////////////////////////
#[cfg(all(feature = "mimalloc", feature = "default"))]
use mimalloc::MiMalloc;

#[cfg(all(feature = "mimalloc", feature = "default"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

////////////////////////Traits//////////////////////////////////
pub trait AsPtr {
    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self as *const _ as *const u8
    }
    #[inline(always)]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self as *mut _ as *mut u8
    }
}

impl AsPtr for [u8] { }
impl AsPtr for &[u8] { }
impl AsPtr for RUMVec<u8> { }
impl AsPtr for &RUMVec<u8> { }
impl AsPtr for RUMString { }

pub trait SizedType {
    #[inline(always)]
    fn size(&self) -> usize;
}

impl SizedType for [u8] { fn size(&self) -> usize { self.len() } }
impl SizedType for &[u8] { fn size(&self) -> usize { self.len() } }
impl SizedType for RUMVec<u8> { fn size(&self) -> usize { self.len() } }
impl SizedType for &RUMVec<u8> { fn size(&self) -> usize { self.len() } }
impl SizedType for RUMString { fn size(&self) -> usize { self.len() } }

pub trait AsSlice: AsPtr + SizedType {
    #[inline(always)]
    fn as_slice(&self) -> &[u8] { as_slice(self.as_ptr(),  self.size()) }
    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [u8] {  as_slice_mut(self.as_mut_ptr(),  self.size()) }

    #[inline(always)]
    fn contains(&self, x: &u8) -> bool {
        self.as_slice().contains(x)
    }
}

impl AsSlice for [u8] { }
impl AsSlice for &[u8] { }
impl AsSlice for RUMVec<u8> { }
impl AsSlice for &RUMVec<u8> { }
impl AsSlice for RUMString { }

#[inline]
pub fn as_slice<'a>(src: *const u8, size: usize) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(src, size) }
}

#[inline]
pub fn as_slice_mut<'a>(src: *mut u8, size: usize) -> &'a mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(src, size) }
}

#[macro_export]
macro_rules! rumtk_mem_quick_array_init {
    ( $typ:ty, $size:expr ) => {{
        const DATA_SLICE_LEN: usize = $size * size_of::<$typ>();
        let arr: [$typ; $size] = unsafe { mem::transmute([0u8; DATA_SLICE_LEN]) };
        arr
    }}
}

/////////////////////////////Copy///////////////////////////////////
#[inline]
pub fn copy_simd_slice<'a, const LANE_SIZE: usize>(src: &[u8], mut dst: &'a mut [u8]) -> &'a mut [u8] {
    let (prefix, middle, postfix) = src.as_simd::<LANE_SIZE>();
    let prefix_len = prefix.len();
    let postfix_len = postfix.len();

    dst[..prefix_len].copy_from_slice(prefix);
    dst = &mut dst[prefix_len..];

    for chunk in middle.into_iter() {
        chunk.copy_to_slice(&mut dst[..LANE_SIZE]);
        dst = &mut dst[LANE_SIZE..];
    }

    dst[..postfix_len].copy_from_slice(postfix);
    dst
}

#[inline]
pub fn copy_from_slice<'a>(src: &[u8], dst: &'a mut [u8]) -> &'a mut [u8] {
    debug_assert!(src.len() <= dst.len(), "Destination memory slice is smaller than source! This is a bug near the call site of copy_from_slice!");
    copy_simd_slice::<CPU_SIMD_64_SIZE>(
        src,
        dst,
    )
}