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
use std::alloc::{AllocError, Allocator};
use std::alloc::{GlobalAlloc, Layout};

use crate::mem::cast_to_nonnull;
use std::ptr::NonNull;

#[cfg(feature = "fast_allocator")]
use mimalloc::MiMalloc;

#[cfg(feature = "fast_allocator")]
static mut SAND: MiMalloc = MiMalloc;

#[cfg(not(feature = "fast_allocator"))]
use std::alloc::System;


#[cfg(not(feature = "fast_allocator"))]
static mut SAND: System = System;

#[inline(always)]
pub unsafe fn direct_alloc(layout: Layout) -> *mut u8 {
    SAND.alloc(layout)
}

#[inline(always)]
pub unsafe fn direct_dealloc(ptr: *mut u8, layout: Layout) {
    SAND.dealloc(ptr, layout)
}

pub struct DirectAllocator;

unsafe impl Allocator for DirectAllocator {
    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { direct_alloc(layout) };
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr, layout.size()) };
        Ok(cast_to_nonnull::<[u8]>(slice))
    }
    #[inline(always)]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        direct_dealloc(ptr.as_ptr(), layout);
    }
}

pub static DIRECT_ALLOCATOR: DirectAllocator = DirectAllocator;

