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
use crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
#[cfg(feature = "fast_allocator")]
use libmimalloc_sys as ffi_mimalloc;
#[cfg(feature = "fast_allocator")]
use mimalloc::MiMalloc;
#[cfg(feature = "fast_allocator")]
use std::alloc::GlobalAlloc;
#[cfg(feature = "fast_allocator")]
use std::alloc::Layout;
#[cfg(feature = "fast_allocator")]
use std::ffi::c_long;
#[cfg(feature = "fast_allocator")]
use std::sync::LazyLock;

#[cfg(feature = "fast_allocator")]
pub type Dune = MiMalloc;

#[cfg(feature = "fast_allocator")]
pub struct Arrakis {
    pub dune: LazyLock<Dune>,
}

#[cfg(feature = "fast_allocator")]
impl Arrakis {
    pub const fn new() -> Self {
        Self {
            dune: LazyLock::new(|| {
                global_reserve_memory(DEFAULT_GLOBAL_MB_ALLOCATION);
                Dune {}
            })
        }
    }
}

#[cfg(feature = "fast_allocator")]
unsafe impl GlobalAlloc for Arrakis {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (*self.dune).alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        (*self.dune).dealloc(ptr, layout)
    }
}

#[cfg(feature = "fast_allocator")]
#[macro_export]
macro_rules! rumtk_dune_new {
    (  ) => {{
        use $crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
        rumtk_dune_new!(DEFAULT_GLOBAL_MB_ALLOCATION)
    }};
    ( $size:expr ) => {{
        use std::sync::LazyLock;
        use $crate::dune::{Dune, Arrakis, global_reserve_memory};
        Arrakis {
            dune: LazyLock::new(|| {
                global_reserve_memory($size);
                Dune {}
            })
        }
    }}
}

#[cfg(feature = "fast_allocator")]
#[inline]
pub fn global_reserve_memory(allocation: usize) {
    // Config MiMalloc before generating instance
    unsafe {
        // Pre-allocate memory to speed up most programs.
        ffi_mimalloc::mi_option_set(ffi_mimalloc::mi_option_reserve_os_memory, allocation as c_long);
    }
}

#[cfg(feature = "fast_allocator")]
#[macro_export]
macro_rules! rumtk_dune_prealloc {
    ( $size:expr ) => {{
        use $crate::dune::global_reserve_memory;
        global_reserve_memory($size);
    }}
}


