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
use std::alloc::{Allocator, GlobalAlloc};
use std::collections::LinkedList;
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

#[macro_export]
macro_rules! rumtk_dune_new {
    ( $capacity:expr ) => {{
        use $crate::dune::dune_allocate;

        dune_allocate($capacity)
    }};
}

#[macro_export]
macro_rules! rumtk_dune_free {
    ( $arena:expr ) => {{
        use $crate::dune::dune_deallocate;

        dune_deallocate($arena)
    }};
}
