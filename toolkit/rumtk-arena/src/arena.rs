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
use memmap2::MmapMut;

pub const ONE_KB: usize = 1024;
pub const ONE_MB: usize = 1024 * ONE_KB;
pub const ONE_GB: usize = 1024 * ONE_MB;
pub const DEFAULT_ARENA_MEMORY_ALLOCATION: usize = 4 * ONE_KB;

///
/// Basic Arena Allocator that uses the crate `mmap2` to request wholesale allocation of memory from
/// the system.
///
/// An arena is a memory management strategy in which you request a chunk of memory upfront and use it
/// to allocate many objects in sequence. Essentially, it turns memory allocation from a heap problem
/// into a stack problem increasing the speed of this process. It is a technique common in the video
/// game industry to minimize the time spent asking the system for allocations.
///
/// Here we offer this small implementation to help speed up parsing operations in other `RUMTK` crates.
/// This is a standalone crate with no dependencies on other `RUMTK` crates.
///
/// Another feature is that we implement the `Allocator` trait thus allowing you to provide an instance
/// of the Arena to other standard collections through the nightly compiler's `allocator_api` feature.
/// Note that this feature is considered unstable.
///
pub struct Arena {
    memory: MmapMut,
    capacity: usize,
    used: usize,
}

impl Arena {
    ///
    /// Allocates a new Arena using the [DEFAULT_ARENA_MEMORY_ALLOCATION] allocation size.
    ///
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_ARENA_MEMORY_ALLOCATION)
    }

    ///
    /// Allocates new Arena with the specified size. At the moment, we use the `mmap2` crate's defaults
    /// for this allocation.
    ///
    pub fn with_capacity(capacity: usize) -> Self {
        let memory = match MmapMut::map_anon(capacity) {
            Ok(m) => m,
            Err(_) => panic!("Failed to map memory"),
        };

        Self {
            memory,
            capacity,
            used: 0,
        }
    }

    pub fn remaining(&self) -> usize {
        self.capacity - self.used
    }

    pub fn can_allocate(&self, size: usize) -> bool {
        let remaining = self.remaining();
        let can_allocate = remaining >= size;
        assert!(can_allocate, "Arena is too small (Requested: {} > Available: {})", size, remaining);
        can_allocate
    }

    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        self.can_allocate(size);

        let ptr = &mut self.memory[self.used..self.used+size];
        self.used += size;
        ptr.as_mut_ptr()
    }

    pub fn write<T>(&mut self, data: T) -> *mut u8
    where
        T: Copy
    {
        let data_length = size_of::<T>();
        let dst = self.allocate(data_length);

        unsafe {
            std::ptr::copy_nonoverlapping(
                std::ptr::addr_of!(data).cast::<u8>(),
                dst,
                data_length,
            );
        }

        dst
    }

    pub fn reset(&mut self) {
        self.used = 0;
    }
}
