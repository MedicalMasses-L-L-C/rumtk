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

use crate::base::*;

pub trait AsRUMPtr {
    fn as_rum_ptr(&self) -> *const u8;
    fn as_mut_rum_ptr(&mut self) -> *mut u8;
}

pub trait SizedRUMType {
    fn size(&self) -> usize;
}

impl SizedRUMType for [u8] { fn size(&self) -> usize { self.len() } }
impl SizedRUMType for &[u8] { fn size(&self) -> usize { self.len() } }
impl SizedRUMType for RUMVec<u8> { fn size(&self) -> usize { self.len() } }
impl SizedRUMType for &RUMVec<u8> { fn size(&self) -> usize { self.len() } }
impl SizedRUMType for RUMString { fn size(&self) -> usize { self.len() } }
impl SizedRUMType for &str { fn size(&self) -> usize { self.len() } }

pub trait AsRUMSlice: AsRUMPtr + SizedRUMType {
    #[inline(always)]
    fn as_rum_slice(&self) -> &[u8] { as_slice(self.as_rum_ptr(),  self.size()) }
    #[inline(always)]
    fn as_rum_slice_mut(&mut self) -> &mut [u8] {  as_slice_mut(self.as_mut_rum_ptr(),  self.size()) }

    #[inline(always)]
    fn contains(&self, x: &u8) -> bool {
        self.as_rum_slice().contains(x)
    }
}

#[inline]
pub fn as_slice<'a>(src: *const u8, size: usize) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(src, size) }
}

#[inline]
pub fn as_slice_mut<'a>(src: *mut u8, size: usize) -> &'a mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(src, size) }
}

/********************** Basic Impl for AsSlice************************/
impl AsRUMPtr for RUMVec<u8> {
    #[inline(always)]
    fn as_rum_ptr(&self) -> *const u8 { self.as_ptr() }
    #[inline(always)]
    fn as_mut_rum_ptr(&mut self) -> *mut u8 { self.as_mut_ptr() }
}
impl AsRUMPtr for &[u8] {
    #[inline(always)]
    fn as_rum_ptr(&self) -> *const u8 { self.as_ptr() }
    #[inline(always)]
    fn as_mut_rum_ptr(&mut self) -> *mut u8 { self as *mut _ as *mut u8 }
}
impl AsRUMPtr for RUMString {
    #[inline(always)]
    fn as_rum_ptr(&self) -> *const u8 { self.as_ptr() }
    #[inline(always)]
    fn as_mut_rum_ptr(&mut self) -> *mut u8 { self.as_mut_ptr() }
}
impl AsRUMPtr for &str {
    #[inline(always)]
    fn as_rum_ptr(&self) -> *const u8 { self.as_ptr() }
    #[inline(always)]
    fn as_mut_rum_ptr(&mut self) -> *mut u8 { self as *mut _ as *mut u8 }
}

/********************** Basic Impl for AsSlice************************/
impl AsRUMSlice for RUMVec<u8> {}
impl AsRUMSlice for &[u8] {}
impl AsRUMSlice for RUMString {}
impl AsRUMSlice for &str {}
