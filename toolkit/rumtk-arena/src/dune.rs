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
use crate::{direct_alloc, direct_dealloc, MemoryPool};
use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::sync::Mutex;

pub struct Arrakis {
    dunes: Mutex<MemoryPool>,
}

impl Arrakis {
    pub const fn with_capacity(allocation_size: usize) -> Self {
        Self { dunes: Mutex::new(MemoryPool::with_chunk_size(allocation_size)) }
    }

    #[inline]
    unsafe fn allocate(&self, layout: Layout) -> *mut u8 {
        let mut dunes = self.dunes.lock().unwrap();
        dunes.allocate(layout)
    }

    #[inline]
    unsafe fn deallocate(&self,ptr: *mut u8, layout: Layout) {
        let mut dunes = self.dunes.lock().unwrap();
        dunes.deallocate(ptr, layout);
    }
}

unsafe impl GlobalAlloc for Arrakis {
    #[cfg(feature = "fast_global_allocator")]
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        direct_alloc(layout.size())
    }

    #[cfg(not(feature = "fast_global_allocator"))]
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
    }
    #[cfg(feature = "fast_global_allocator")]
    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        direct_dealloc(ptr, _layout)
    }

    #[cfg(not(feature = "fast_global_allocator"))]
    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.deallocate(ptr, _layout)
    }
}

#[macro_export]
macro_rules! rumtk_dune_new {
    (  ) => {{
        use $crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
        rumtk_dune_new!(DEFAULT_GLOBAL_MB_ALLOCATION)
    }};
    ( $size:expr ) => {{
        use std::sync::LazyLock;
        use $crate::dune::{Arrakis};
        use $crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
        Arrakis::with_capacity(DEFAULT_GLOBAL_MB_ALLOCATION)
    }}
}


