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

/*
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
*/

pub trait AsSlice {
    type Item;
    fn as_slice(&self) -> &[Self::Item];
    fn as_slice_mut(&self) -> &mut [Self::Item];
}

#[inline]
pub fn as_slice<'a>(src: *const u8, size: usize) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(src, size) }
}

#[inline]
pub fn as_slice_mut<'a>(src: *mut u8, size: usize) -> &'a mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(src, size) }
}

#[inline]
pub fn copy_from_slice<'a>(src: &[u8], dst: &'a mut [u8]) -> &'a mut [u8] {
    assert!(src.len() <= dst.len(), "Destination memory slice is smaller than source! This is a bug near the call site of copy_from_slice!");
    dst.copy_from_slice(src);
    dst
}