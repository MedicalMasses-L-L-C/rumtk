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
use std::alloc::{AllocError, Allocator};
use std::alloc::{GlobalAlloc, Layout};
use std::cell::RefCell;
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
/// let mut arena = Arena::new();
/// let result_ptr = arena.write(5);
///
/// ```
///
/// ### Usage with a Vector.
/// ```
/// #![feature(allocator_api)]
/// use crate::rumtk_arena::Arena;
///
/// let mut arena = Arena::new();
/// let mut v = Vec::<usize, &Arena>::with_capacity_in(5, &arena);
/// v.push(5);
///
/// ```
///
#[derive(Debug)]
pub struct ArenaAlloc {
    memory: MmapMut,
    capacity: usize,
    used: usize,
}

impl ArenaAlloc {
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

    ///
    /// Provides the remaining `uncommitted` number of bytes. This represents the number of bytes left
    /// to add more objects.
    ///
    #[inline(always)]
    pub fn remaining(&self) -> usize {
        self.capacity - self.used
    }

    ///
    /// Checks if it is possible to allocate the next object. This is an assertion guarded operation and will
    /// `panic`!!!!!!!
    ///
    #[inline(always)]
    pub fn can_allocate(&self, size: usize) -> ArenaResult<bool> {
        let remaining = self.remaining();
        let can_allocate = remaining >= size;
        if can_allocate {
            Ok(can_allocate)
        } else {
            Err(AllocError)
        }
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
        self.can_allocate(size)?;

        let ptr = &mut self.memory[self.used..self.used+size];
        self.used += size;
        Ok(ptr)
    }

    ///
    /// Grows the allocated memory. Basically, we advance the pointer by the difference
    ///
    #[inline(always)]
    pub fn grow(&mut self, old_size: usize, new_size: usize) -> ArenaResult<*mut [u8]> {
        self.uncommit(old_size);
        self.commit(new_size)
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
        self.used -= length;
    }

    ///
    /// Resets the internal cursor. No real deallocations occur!
    ///
    #[inline(always)]
    pub fn reset(&mut self) {
        self.used = 0;
    }

    pub fn address(&self) -> ArenaBaseAddress {
        self.memory.as_ptr()
    }

    pub fn is_empty(&self) -> bool {
        self.used == 0
    }
}

impl Default for ArenaAlloc {
    fn default() -> Self {
        Self::new()
    }
}


///
/// Arena Allocator wrapper with interior mutability that uses the crate `memmap2` to request
/// wholesale allocation of memory from the system.
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
/// let mut arena = Arena::new();
/// let result_ptr = arena.write(5);
///
/// ```
///
/// ### Usage with a Vector.
/// ```
/// #![feature(allocator_api)]
/// use crate::rumtk_arena::Arena;
///
/// let mut arena = Arena::new();
/// let mut v = Vec::<usize, &Arena>::with_capacity_in(5, &arena);
/// v.push(5);
///
/// ```
///
#[derive(Debug, Default)]
pub struct Arena {
    memory: RefCell<ArenaAlloc>
}
impl Arena {
    pub fn new() -> Self {
        Self {
            memory: RefCell::new(ArenaAlloc::new())
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory: RefCell::new(ArenaAlloc::with_capacity(capacity))
        }
    }

    pub fn commit(&self, size: usize) -> ArenaResult<*mut [u8]> {
        self.memory.borrow_mut().commit(size)
    }

    pub fn grow_block(&self, old_size: usize, new_size: usize) -> ArenaResult<*mut [u8]> {
        self.memory.borrow_mut().grow(old_size, new_size)
    }

    pub fn write<T>(&self, data: T) -> ArenaResult<NonNull<T>> {
        self.memory.borrow_mut().write(data)
    }

    pub fn uncommit(&self, length: usize) {
        self.memory.borrow_mut().uncommit(length)
    }

    pub fn reset(&self) {
        self.memory.borrow_mut().reset()
    }

    pub fn remaining(&self) -> usize {
        self.memory.borrow().remaining()
    }

    pub fn address(&self) -> ArenaBaseAddress {
        self.memory.borrow().address()
    }

    pub fn is_empty(&self) -> bool {
        self.memory.borrow().is_empty()
    }
}

unsafe impl Allocator for Arena {
    // Required methods
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let r = self.commit(layout.size())?;
        let nz_r = cast_to_nonnull(r);
        Ok(nz_r)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.uncommit(layout.size());
    }

    // Provided methods
    fn allocate_zeroed(
        &self,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let length = layout.size();
        let allocated = self.commit(length)?;

        zero_memory(allocated, 0, length);

        Ok(cast_to_nonnull(allocated))
    }
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new_ptr = self.grow_block(old_layout.size(), new_layout.size())?;
        let nz_new_ptr = cast_to_nonnull(new_ptr);
        Ok(nz_new_ptr)
    }
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new_ptr = zero_memory(self.grow_block(old_layout.size(), new_layout.size())?, old_layout.size(), new_layout.size());
        Ok(cast_to_nonnull(new_ptr))
    }
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.uncommit(old_layout.size());
        Ok(cast_to_nonnull(self.commit(new_layout.size())?))
    }
    fn by_ref(&self) -> &Self
    where Self: Sized { &self }
}

unsafe impl GlobalAlloc for Arena {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.commit(layout.size()).unwrap().as_mut_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.uncommit(layout.size());
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
}
