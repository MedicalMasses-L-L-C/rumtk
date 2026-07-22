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
#[cfg(feature = "fast_allocator")]
use crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
#[cfg(feature = "fast_allocator")]
use libmimalloc_sys as ffi_mimalloc;
#[cfg(feature = "fast_allocator")]
use mimalloc::MiMalloc;
#[cfg(feature = "fast_allocator")]
use std::alloc::GlobalAlloc;
#[cfg(feature = "fast_allocator")]
use std::ffi::c_long;

#[cfg(feature = "fast_allocator")]
pub type GlobalDune = MiMalloc;

#[cfg(feature = "fast_allocator")]
pub struct GlobalDuneBuilder {
    pub pre_allocate: usize,
    pub pre_allocate_huge: usize,
    pub enable_large_pages: bool,
}

#[cfg(feature = "fast_allocator")]
impl GlobalDuneBuilder {
    #[inline]
    pub const fn new() -> Self {
        Self {
            pre_allocate: DEFAULT_GLOBAL_MB_ALLOCATION,
            pre_allocate_huge: 0,
            enable_large_pages: true,
        }
    }

    #[inline]
    pub const fn pre_allocate(mut self, size: usize) -> Self {
        self.pre_allocate = size;
        self
    }

    #[inline]
    pub const fn pre_allocate_huge(mut self, size: usize) -> Self {
        self.pre_allocate_huge = size;
        self
    }

    #[inline]
    pub const fn enable_large_pages(mut self, v: bool) -> Self {
        self.enable_large_pages = v;
        self
    }

    #[inline]
    pub const fn build(self) -> GlobalDune {
        // Config MiMalloc before generating instance
        unsafe {
            // Pre-allocate memory to speed up most programs.
            ffi_mimalloc::mi_option_set(ffi_mimalloc::mi_option_reserve_os_memory, self.pre_allocate as c_long);

            // enable the Large Page support
            ffi_mimalloc::mi_option_set_enabled(ffi_mimalloc::mi_option_large_os_pages, self.enable_large_pages);

            // enable the Huge Page support
            ffi_mimalloc::mi_option_set(ffi_mimalloc::mi_option_reserve_huge_os_pages, self.pre_allocate_huge as c_long);
        }

        GlobalDune {}
    }
}
