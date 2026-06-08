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
use crate::Arena;
use std::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use std::collections::LinkedList;
use std::ptr::NonNull;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

pub type SafeArena = Arc<RwLock<Arena>>;
type Sand = LinkedList<Arena>;
type DuneLock = Arc<Mutex<u8>>;
type ArrakisDunes = LazyLock<Sand>;
type ArrakisLock = LazyLock<DuneLock>;

static mut ARRAKIS: ArrakisDunes = ArrakisDunes::new(|| Sand::default());
static mut LOCK: ArrakisLock = ArrakisLock::new(|| Arc::new(Mutex::new(0)));

pub fn dune_allocate(chunk_size: usize) -> &'static Arena {
    unsafe {
        let _unused = LOCK.lock().unwrap();
        ARRAKIS.push_back_mut(Arena::with_capacity(chunk_size))
    }
}

pub fn dune_deallocate(arena: &'static Arena) {
    let address = arena.address();
    unsafe {
        let _unused = LOCK.lock().unwrap();

        ARRAKIS.retain(|e| e.address() != address);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dune {
    arena: &'static Arena,
}

impl Dune {
    pub fn new(chunk_size: usize) -> Self {
        let arena = dune_allocate(chunk_size);

        Self { arena }
    }

    pub fn arena(&self) -> &Arena {
        self.arena
    }

    pub fn drop(&mut self) {
       dune_deallocate(self.arena);
    }
}

unsafe impl Allocator for Dune {
    // Required methods
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.arena.allocate(layout)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.arena.deallocate(ptr, layout)
    }

    // Provided methods
    fn allocate_zeroed(
        &self,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.arena.allocate_zeroed(layout)
    }
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.arena.grow(ptr, old_layout, new_layout)
    }
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.arena.grow_zeroed(ptr, old_layout, new_layout)
    }
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.arena.shrink(ptr, old_layout, new_layout)
    }
    fn by_ref(&self) -> &Self
    where Self: Sized { &self }
}

#[macro_export]
macro_rules! rumtk_arena_allocate {
    ( $capacity:expr ) => {{
        use $crate::Dune;

        Dune::new($capacity)
    }};
}
