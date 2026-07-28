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
use std::ptr::NonNull;

#[inline(always)]
pub fn cast_to_nonnull<T: ?Sized>(dst: *mut T) -> NonNull<T> {
    match NonNull::new(dst) {
        Some(ptr) => ptr,
        None => panic!("Failed to allocate memory"),
    }
}

#[inline(always)]
pub fn cast_data_to_ptr<T>(data: &T) -> *const u8 {
    std::ptr::addr_of!(*data).cast::<u8>()
}

#[inline(always)]
pub fn sizeof<T>(data: &T) -> usize {
    size_of::<T>()
}

#[inline(always)]
pub fn zero_memory(data: *mut [u8], offset: usize, length: usize) -> *mut [u8] {
    let chunk = unsafe { &mut *data };
    for i in offset..offset + length {
        chunk[i] = 0;
    }

    data
}
