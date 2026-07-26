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
use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::collections::LinkedList;

use crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
use crate::Arena;
use crate::{direct_alloc, DirectAllocator, DIRECT_ALLOCATOR};
use crate::{rumtk_arena_new, AsPtr};

type DuneLL = LinkedList<Arena, &'static DirectAllocator>;

static mut BUFFER: Arena = Arena::null();
static mut BUFFER_VIEW: Arena = Arena::null();
static mut DUNES: DuneLL = DuneLL::new_in(&DIRECT_ALLOCATOR);
static mut ALLOC_SIZE: usize = DEFAULT_GLOBAL_MB_ALLOCATION;

unsafe fn is_empty() -> bool {
    remaining() <= 0
}

unsafe fn is_initialized() -> bool {
    !is_empty()
}

unsafe fn remaining() -> usize {
    BUFFER_VIEW.remaining()
}

unsafe fn init_dunes(alloc_size: usize) {
    ALLOC_SIZE = alloc_size;
    let ptr = direct_alloc(ALLOC_SIZE);
    BUFFER = rumtk_arena_new!(ptr, ALLOC_SIZE, true);
    BUFFER_VIEW = BUFFER.freeze();
}

unsafe fn allocate(requested: usize, alloc_size: usize) -> *mut u8 {
    if is_empty() {
        init_dunes(alloc_size);
    }

    let slot = BUFFER_VIEW.split_to(requested);
    DUNES.push_back(slot);
    DUNES.back_mut().unwrap().as_mut_ptr()
}

unsafe fn deallocate(ptr: *mut u8, _layout: Layout) {
    //DUNES.uncommit(_layout.size());
}

pub struct Arrakis {
    pub alloc_size: usize,
}

impl Arrakis {
    pub const fn new(allocation_size: usize) -> Self {
        Self { alloc_size: allocation_size }
    }
}

unsafe impl GlobalAlloc for Arrakis {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocate(layout.size(), self.alloc_size)
        //direct_alloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        deallocate(ptr, _layout)
        //direct_dealloc(ptr, _layout.size())
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
        Arrakis::new(DEFAULT_GLOBAL_MB_ALLOCATION)
    }}
}


