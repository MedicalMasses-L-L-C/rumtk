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
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::Arena;
use crate::{direct_alloc, DirectAllocator};
use crate::{rumtk_arena_new, DIRECT_ALLOCATOR};

type Dune_LL = LinkedList<Arena, &'static DirectAllocator>;

pub struct Dune {
    pub sand: LazyLock<Dune_LL>,
    pub allocation_size: usize,
}

impl Dune {
    pub const fn new(allocation_size: usize) -> Self {
        Self {
            sand: LazyLock::new(|| Dune_LL::new_in(&DIRECT_ALLOCATOR)),
            allocation_size,
        }
    }

    fn cache_retrieve(&mut self) -> &mut Dune_LL {
        &mut (*self.sand)
    }

    unsafe fn current_arena(&mut self) -> &mut Arena {
        let cache = self.cache_retrieve();
        cache.back_mut().unwrap()
    }

    unsafe fn is_initialized(&mut self) -> bool {
        let cache = self.cache_retrieve();
        !cache.is_empty()
    }

    unsafe fn new_sand(&mut self) -> &mut Arena {
        let alloc_size = self.allocation_size;
        let cache = self.cache_retrieve();
        let ptr = direct_alloc(alloc_size);
        let new_arena = rumtk_arena_new!(ptr, alloc_size, true);
        cache.push_back(new_arena);
        cache.back_mut().unwrap()
    }

    unsafe fn sand_get(&mut self, min_required_size: usize) -> &mut Arena {
        let mut current = match self.is_initialized() {
            true => self.current_arena(),
            false => self.new_sand(),
        };
        if current.remaining() < min_required_size {
            current = self.new_sand();
        }

        let slot = current.split_to(min_required_size);
        let cache = self.cache_retrieve();
        cache.push_back(slot);
        cache.back_mut().unwrap()
    }

    pub unsafe fn allocate(&mut self, size: usize) -> *mut u8 {
        let arena = self.sand_get(size);
        arena.commit(size).unwrap() as *mut u8
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        let mut cache = self.cache_retrieve();
        cache.retain(|slot| slot.address() != ptr);
    }
}

#[cfg(feature = "fast_allocator")]
pub struct Arrakis {
    pub dunes: Mutex<Dune>,
}

#[cfg(feature = "fast_allocator")]
impl Arrakis {
    pub const fn new(allocation_size: usize) -> Self {
        Self {
            dunes: Mutex::new(Dune::new(allocation_size)),
        }
    }
}

#[cfg(feature = "fast_allocator")]
unsafe impl GlobalAlloc for Arrakis {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.dunes.lock().unwrap().allocate(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.dunes.lock().unwrap().deallocate(ptr)
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
        use $crate::dune::{Dune, Arrakis};
        use $crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
        Arrakis::new(DEFAULT_GLOBAL_MB_ALLOCATION)
    }}
}

#[cfg(feature = "fast_allocator")]
#[macro_export]
macro_rules! rumtk_dune_prealloc {
    ( $size:expr ) => {{
        use $crate::dune::global_reserve_memory;
        global_reserve_memory($size);
    }}
}


