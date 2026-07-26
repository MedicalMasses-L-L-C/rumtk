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
use crate::buffers::RUMBuffer;
use crate::direct_alloc;
use crate::mem::{as_slice, as_slice_mut, AsPtr, AsSlice, SizedType};
use std::alloc::AllocError;
use std::ops::Index;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo, RangeToInclusive};
use std::ptr::NonNull;

pub const ONE_KB: usize = 1024;
pub const ONE_MB: usize = 1024 * ONE_KB;
pub const ONE_GB: usize = 1024 * ONE_MB;
pub const DEFAULT_ARENA_MEMORY_ALLOCATION: usize = 4 * ONE_KB;

#[inline(always)]
pub fn cast_to_nonnull<T: ?Sized>(dst: *mut T) -> NonNull<T> {
    match NonNull::new(dst) {
        Some(ptr) => ptr,
        None => panic!("Failed to allocate memory"),
    }
}

#[inline(always)]
pub fn cast_data_to_ptr<T>(data: &T) -> *const u8 {
    std::ptr::addr_of!(*data).cast::<u8>()
}

#[inline(always)]
pub fn get_data_length<T>(data: &T) -> usize {
    size_of::<T>()
}

#[inline(always)]
pub fn zero_memory(data: *mut [u8], offset: usize, length: usize) -> *mut [u8] {
    let chunk = unsafe { &mut *data };
    for i in offset..offset + length {
        chunk[i] = 0;
    }

    data
}

pub type ArenaResult<T> = Result<T, AllocError>;
pub type ArenaBaseAddress = *const u8;

///
/// Basic Arena Allocator that uses the crate `memmap2` to request wholesale allocation of memory from
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
/// ## Safety
///
/// * Calling `reset` simply resets the pointer to 0 and thus technically allows for the potential to
/// leak a prior round of work's information if a pointer return by `allocate` is misused.
/// * No calls to drop are invoked!!! You have to find a different way to manually do so. This implementation
/// is meant to deal with quick allocation needs and not with self managed resources for which a RAII
/// approach might be more appropriate.
///
/// ## Example
///
/// ### Simple initialization and Writing of value.
/// ```
/// use crate::rumtk_arena::Arena;
///
/// let mut arena = Arena::with_capacity(size_of::<usize>() * 1);
/// let result_ptr = arena.write(5);
///
/// ```
///
#[derive(Debug)]
pub struct Arena {
    memory: RUMBuffer,
    remaining: usize,
    capacity: usize,
}

impl Arena {
    ///
    /// Allocates a new Arena using the [DEFAULT_ARENA_MEMORY_ALLOCATION] allocation size.
    ///
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_ARENA_MEMORY_ALLOCATION)
    }

    ///
    /// Allocates new Arena with the specified size. At the moment, we use the `memmap2` crate's defaults
    /// for this allocation.
    ///
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory: RUMBuffer::from_parts(unsafe { direct_alloc(capacity) }, capacity, true),
            remaining: capacity,
            capacity,
        }
    }

    #[inline]
    pub const fn null() -> Self {
        Self {
            memory: RUMBuffer::new(),
            remaining: 0,
            capacity: 0,
        }
    }

    #[inline]
    pub fn from_parts(ptr: *mut u8, capacity: usize, dealloc: bool) -> Self {
        Self {
            memory: RUMBuffer::from_parts(ptr, capacity, dealloc),
            remaining: capacity,
            capacity,
        }
    }

    #[inline]
    pub fn split_to(&mut self, len: usize) -> Self {
        match self.memory.split_to(len) {
            Some(new_buffer) => {
                self.remaining -= len;
                self.capacity -= len;

                Self {
                    memory: new_buffer,
                    remaining: len,
                    capacity: len,
                }
            },
            None => {
                Self {
                    memory: RUMBuffer::new(),
                    remaining: 0,
                    capacity: 0,
                }
            }
        }
    }

    #[inline]
    pub fn freeze(&mut self) -> Self {
        Self {
            memory: self.memory.freeze(),
            remaining: self.remaining,
            capacity: self.capacity,
        }
    }

    #[inline(always)]
    pub fn remaining(&self) -> usize {
        self.remaining
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    ///
    /// Checks if it is possible to allocate the next object. This is an assertion guarded operation and will
    /// `panic`!!!!!!!
    ///
    #[inline(always)]
    pub fn can_allocate(&self, size: usize) -> bool {
        let remaining = self.remaining();
        remaining >= size
    }

    ///
    /// Commits a chunk of memory from our memory pool.
    ///
    /// ## Safety
    ///
    /// We call [Self::can_allocate] to assert that the size requested does not exceed the total
    /// pool available. `panic` if we do not have enough memory to commit.
    ///
    #[inline(always)]
    pub fn commit(&mut self, size: usize) -> ArenaResult<*mut [u8]> {
        if self.can_allocate(size) {
            let lower_bound = self.capacity - self.remaining;
            let upper_bound = lower_bound + size;
            let slice = &mut self.memory[lower_bound..upper_bound];
            self.remaining -= size;
            Ok(slice)
        } else {
            eprintln!("Cannot allocate {} bytes due to lack of space!", size);
            Err(AllocError)
        }
    }

    ///
    /// Writes a number of bytes into a pre allocated segment from our pool.
    ///
    pub fn write_bytes(&mut self, src: *const u8, data_length: usize) -> ArenaResult<*mut [u8]> {
        let dst = self.commit(data_length)?;
        unsafe {
            std::ptr::copy_nonoverlapping(
                src,
                dst.as_mut_ptr(),
                data_length,
            );
        }
        Ok(dst)
    }

    ///
    /// Commits a type object into the memory advancing the internal cursor.
    ///
    /// ## Order of Operations
    /// 1. Calculate size of object.
    /// 2. Commit a chunk of memory via [Self::commit].
    /// 3. Cast object to a byte pointer.
    /// 4. Memcopy from `src` to `dst` by the number of bytes calculated in #1.
    ///
    /// ## Safety
    ///
    /// We call [Self::commit] first before applying a memcopy. [Self::commit] can panic if there is a bug in
    /// this crate due to our call of `assert`!
    ///
    /// Panics if casting to non null pointer somehow fails.
    ///
    pub fn write<T>(&mut self, data: T) -> ArenaResult<NonNull<T>> {
        let data_length = size_of::<T>();
        let src = std::ptr::addr_of!(data).cast::<u8>();

        let mem = cast_to_nonnull(self.write_bytes(src, data_length)?);
        Ok(mem.cast())
    }

    ///
    /// We do not truly drop objects. Instead, we move the cursor back by the requested number of bytes.
    ///
    /// ## Safety
    ///
    /// Note that this means old results remain valid and could accidentally end up in a new allocation
    /// that could be safety sensitive.
    ///
    #[inline(always)]
    pub fn uncommit(&mut self, length: usize) {
        let new_lower_bound = self.remaining() - (length % self.len());
        self.remaining = new_lower_bound;
    }

    ///
    /// Resets the internal cursor. No real deallocations occur!
    ///
    #[inline(always)]
    pub fn reset(&mut self) {
        self.remaining = self.capacity;
    }

    #[inline(always)]
    pub fn address(&self) -> ArenaBaseAddress {
        self.as_ptr()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.capacity()
    }
}

impl AsSlice for Arena {
    #[inline(always)]
    fn as_slice(&self) -> &'static [u8] { as_slice(self.as_ptr(),  self.size()) }
    #[inline(always)]
    fn as_slice_mut(&mut self) -> &'static mut [u8] {  as_slice_mut(self.as_mut_ptr(),  self.size()) }

    #[inline(always)]
    fn contains(&self, x: &u8) -> bool {
        self.as_slice().contains(x)
    }
}

impl AsPtr for Arena {
    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.memory.as_ptr()
    }
    #[inline(always)]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.memory.as_mut_ptr()
    }
}

impl SizedType for Arena {
    #[inline(always)]
    fn size(&self) -> usize {
        self.capacity
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Arena {}
unsafe impl Sync for Arena {}

impl Index<usize> for Arena {
    type Output = u8;
    #[inline]
    fn index(&self, i: usize) -> & Self::Output {
        &self.as_slice()[i]
    }
}

impl Index<Range<usize>> for Arena {
    type Output = [u8];
    #[inline]
    fn index(&self, i: Range<usize>) -> & Self::Output {
        &self.as_slice()[i.start..i.end]
    }
}

impl Index<RangeTo<usize>> for Arena {
    type Output = [u8];
    #[inline]
    fn index(&self, i: RangeTo<usize>) -> & Self::Output {
        &self.as_slice()[..i.end]
    }
}

impl Index<RangeFrom<usize>> for Arena {
    type Output = [u8];
    #[inline]
    fn index(&self, i: RangeFrom<usize>) -> & Self::Output {
        &self.as_slice()[i.start..]
    }
}

impl Index<RangeToInclusive<usize>> for Arena {
    type Output = [u8];
    #[inline]
    fn index(&self, i: RangeToInclusive<usize>) -> & Self::Output {
        &self.as_slice()[..=i.end]
    }
}

impl Index<RangeFull> for Arena {
    type Output = [u8];
    #[inline]
    fn index(&self, i: RangeFull) -> & Self::Output {
        self.as_slice()
    }
}

#[macro_export]
macro_rules! rumtk_arena_new {
    (  ) => {{
        use $crate::arena::Arena;
        Arena::new()
    }};
    ( $capacity:expr ) => {{
        use $crate::arena::Arena;

        Arena::with_capacity($capacity)
    }};
    ( $ptr:expr, $capacity:expr, $dealloc:expr ) => {{
        use $crate::arena::Arena;

        Arena::from_parts($ptr, $capacity, $dealloc)
    }};
}
