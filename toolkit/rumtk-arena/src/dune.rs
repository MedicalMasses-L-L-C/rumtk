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
use crate::arena::ArenaResult;
use crate::Arena;
use std::alloc::{Allocator, GlobalAlloc};
use std::borrow::Borrow;
use std::collections::LinkedList;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

pub type SafeArena = Arc<RwLock<Arena>>;
type Sand = LinkedList<Arena>;
type DuneLock = Arc<Mutex<u8>>;
type NullDune = LazyLock<Arena>;
type ArrakisDunes = LazyLock<Sand>;
type ArrakisLock = LazyLock<DuneLock>;

static mut NULL: NullDune = NullDune::new(|| Arena::new());
static mut ARRAKIS: ArrakisDunes = ArrakisDunes::new(|| Sand::default());
static mut LOCK: ArrakisLock = ArrakisLock::new(|| Arc::new(Mutex::new(0)));

#[inline]
fn dune_allocate(chunk_size: usize) -> &'static Arena {
    unsafe {
        let _unused = LOCK.lock().unwrap();
        ARRAKIS.push_back_mut(Arena::with_capacity(chunk_size))
    }
}

#[inline]
fn dune_deallocate(arena: &'static Arena) {
    let address = arena.address();
    unsafe {
        let _unused = LOCK.lock().unwrap();

        ARRAKIS.retain(|e| e.address() != address);
    }
}

#[derive(Debug, Clone)]
pub struct Dune {
    pub arena: &'static Arena,
}

impl Dune {
    pub fn new() -> Self {
        Self {
            arena: unsafe { &*NULL },
        }
    }

    pub fn with_capacity(chunk_size: usize) -> Self {
        Self {
            arena: dune_allocate(chunk_size),
        }
    }

    pub fn allocate_raw(&mut self, size: usize) -> ArenaResult<*mut u8> {
        Ok(self.arena.commit(size)? as *mut u8)
    }

    pub fn allocate_const_raw(&mut self, size: usize) -> ArenaResult<*const u8> {
        Ok(self.arena.commit(size)? as *const u8)
    }
}

impl Default for Dune {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Dune {
    fn drop(&mut self) {
        dune_deallocate(self.arena);
    }
}

#[macro_export]
macro_rules! rumtk_dune_new {
    (  ) => {{
        use $crate::dune::Dune;

        Dune::default()
    }};
    ( $capacity:expr ) => {{
        use $crate::dune::Dune;

        Dune::with_capacity($capacity)
    }};
}
