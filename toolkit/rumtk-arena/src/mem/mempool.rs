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
use std::alloc::Layout;
use std::collections::LinkedList;
use std::ptr::null_mut;

use crate::constants::DEFAULT_GLOBAL_MB_ALLOCATION;
use crate::mem::alloc::{direct_alloc, direct_dealloc, DirectAllocator, DIRECT_ALLOCATOR};

///
/// Smallest slot size handed out by the pool. Every slot is a power of two and at least this big,
/// so every slot boundary within a [Chunk] stays a multiple of this value.
///
pub const MIN_SLOT_SIZE: usize = 4;

///
/// Bookkeeping list used to track the allocated [Chunk] instances of a [MemoryPool].
///
/// Nodes are allocated with the [DIRECT_ALLOCATOR] so bookkeeping never goes through the global
/// `alloc`/`dealloc` machinery. This keeps the pool safe for use from within a global allocator.
///
pub type ChunkList = LinkedList<Chunk, &'static DirectAllocator>;

///
/// Bookkeeping list used to track the sections of continuous free memory within a [Chunk].
///
/// The list is kept sorted by address so adjacent freed sections can be merged back together
/// (defragmented) in a single pass. Nodes are allocated with the [DIRECT_ALLOCATOR].
///
pub type FreeList = Vec<FreeSlot, &'static DirectAllocator>;

///
/// A section of continuous free memory within a [Chunk].
///
#[derive(Debug, Copy, Clone)]
pub struct FreeSlot {
    ptr: *mut u8,
    size: usize,
}

///
/// A wholesale block of memory obtained via [direct_alloc] out of which the [MemoryPool] carves
/// the `*mut u8` slots handed to consumers.
///
/// A [Chunk] serves allocations in two ways.
/// 1. Reusing a deallocated section tracked in its [FreeList] (after defragmenting it).
/// 2. Bumping the internal cursor over the untouched tail of the block.
///
/// The backing block is released via [direct_dealloc] when the [Chunk] is dropped.
///
#[derive(Debug)]
pub struct Chunk {
    base: *mut u8,
    capacity: usize,
    cursor: usize,
    free_slots: FreeList,
}

impl Chunk {
    ///
    /// Allocates a new [Chunk] of `capacity` bytes using [direct_alloc].
    ///
    /// Returns [None] if the system refuses to hand us the memory.
    ///
    pub fn new(capacity: usize) -> Option<Self> {
        let base = unsafe { direct_alloc(capacity) };
        if base.is_null() {
            return None;
        }
        Some(Self {
            base,
            capacity,
            cursor: 0,
            free_slots: FreeList::with_capacity_in(1024, &DIRECT_ALLOCATOR),
        })
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    ///
    /// Number of bytes remaining in the untouched tail of the [Chunk].
    ///
    #[inline(always)]
    pub fn remaining(&self) -> usize {
        self.capacity - self.cursor
    }

    ///
    /// Checks if `ptr` points into the memory block owned by this [Chunk].
    ///
    #[inline(always)]
    pub fn contains(&self, ptr: *const u8) -> bool {
        let addr = ptr as usize;
        let base = self.base as usize;
        addr >= base && addr < base + self.capacity
    }

    ///
    /// Checks if this [Chunk] has a slot of `size` bytes aligned to `align` available, either in
    /// its [FreeList] or in its untouched tail. Call [Self::defragment] first for best results.
    ///
    #[inline(always)]
    pub fn can_allocate(&self, size: usize, align: usize) -> bool {
        self.can_bump(size, align) || self.has_free_slot(size, align)
    }

    ///
    /// Merges adjacent [FreeSlot] entries back into single continuous sections.
    ///
    /// Because the [FreeList] is kept sorted by address, a single pass suffices. Merging maximizes
    /// the odds that a deallocated region can be recycled for a new allocation.
    ///
    #[inline]
    pub fn defragment(&mut self) {
        let mut i = 0;
        while i < self.free_slots.len() {
            let (ptr, size) = (self.free_slots[i].ptr, self.free_slots[i].size);

            let merge_size = match self.free_slots.get(i + 1) {
                Some(next) if ptr.wrapping_add(size) == next.ptr => Some(next.size),
                None => break,
                _ => None,
            };
            match merge_size {
                Some(extra) => {
                    self.free_slots.remove(i + 1);
                    self.free_slots[i].size += extra;
                }
                None => {
                    i += 1;
                },
            }
        }
    }

    ///
    /// Obtains a `*mut u8` slot of `size` bytes aligned to `align` from this [Chunk].
    ///
    /// ## Order of Operations
    /// 1. Defragment the [FreeList] via [Self::defragment].
    /// 2. Try to recycle a deallocated section via [Self::reclaim].
    /// 3. Fall back to bumping the cursor over the untouched tail via [Self::bump].
    ///
    /// Returns [None] if no slot is available in this [Chunk].
    ///
    #[inline]
    pub fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        match self.bump(size, align) {
            Some(ptr) => Some(ptr),
            None => {
                self.defragment();
                match self.reclaim(size, align) {
                    Some(ptr) => Some(ptr),
                    None => self.bump(size, align),
                }
            }
        }
    }

    ///
    /// Returns a slot to this [Chunk] by recording it in the [FreeList] so a later allocation can
    /// recycle it. No memory is returned to the system here.
    ///
    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        self.release(ptr, size);
    }

    #[inline(always)]
    fn aligned(ptr: *const u8, align: usize) -> bool {
        (ptr as usize) & (align - 1) == 0
    }

    #[inline(always)]
    fn padding(ptr: *const u8, align: usize) -> usize {
        (align - ((ptr as usize) & (align - 1))) & (align - 1)
    }

    #[inline(always)]
    fn has_free_slot(&self, size: usize, align: usize) -> bool {
        self.free_slots
            .iter()
            .any(|slot| slot.size >= size && Self::aligned(slot.ptr, align))
    }

    #[inline(always)]
    fn can_bump(&self, size: usize, align: usize) -> bool {
        let addr = unsafe { self.base.add(self.cursor) };
        match Self::padding(addr, align).checked_add(size) {
            Some(needed) => self.remaining() >= needed,
            None => false,
        }
    }

    ///
    /// Recycles a deallocated section from the [FreeList]. The winning [FreeSlot] is shrunk by
    /// `size` bytes and removed entirely once exhausted.
    ///
    #[inline]
    fn reclaim(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        for i in 0.. self.free_slots.len() {
            let slot = &mut self.free_slots[i];
            if slot.size >= size && Self::aligned(slot.ptr, align) {
                let ptr = slot.ptr;
                slot.ptr = unsafe { slot.ptr.add(size) };
                slot.size -= size;
                if slot.size == 0 {
                    self.free_slots.remove(i);
                }
                return Some(ptr);
            }
        }
        None
    }

    ///
    /// Carves a slot out of the untouched tail of the [Chunk]. Any bytes skipped to satisfy
    /// `align` are recorded in the [FreeList] so they can be recycled later.
    ///
    #[inline]
    fn bump(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        if !self.can_bump(size, align) {
            return None;
        }

        let addr = unsafe { self.base.add(self.cursor) };
        let pad = Self::padding(addr, align);
        let needed = pad.checked_add(size)?;
        if self.remaining() < needed {
            return None;
        }
        if pad > 0 {
            self.release(addr, pad);
        }
        self.cursor += needed;
        Some(unsafe { addr.add(pad) })
    }

    ///
    /// Inserts a section into the [FreeList] keeping the list sorted by address so
    /// [Self::defragment] can merge adjacent sections in a single pass.
    ///
    #[inline]
    fn release(&mut self, ptr: *mut u8, size: usize) {
        for i in 0.. self.free_slots.len() {
            let slot = &mut self.free_slots[i];
            if slot.ptr < ptr {
                continue;
            }

            self.free_slots.insert(i, FreeSlot { ptr, size });
        }
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe { direct_dealloc(self.base, self.capacity) };
    }
}

///
/// Memory pool manager that preallocates memory in chunks of [DEFAULT_GLOBAL_MB_ALLOCATION] bytes
/// and hands out `*mut u8` pointers whose sizes are rounded up to the next power of two.
///
/// ## Order of Operations
/// 1. Round the [Layout] size up to a power of two (at least [MIN_SLOT_SIZE]).
/// 2. Search the allocated [Chunk] list for one with an available slot via
///    [Self::find_available_chunk]. Each candidate first defragments its deallocated sections so
///    adjacent freed slots merge back into continuous memory that can be recycled.
/// 3. If no [Chunk] has a slot available, allocate a new [Chunk] of at least
///    [DEFAULT_GLOBAL_MB_ALLOCATION] bytes and serve the request from it.
///
/// Deallocated pointers are recorded per [Chunk] as sections of continuous free memory and get
/// recycled by later allocations. Chunk memory is only returned to the system when the
/// [MemoryPool] itself is dropped.
///
/// ## Safety
///
/// * No calls to drop are invoked on the objects living inside the slots! This pool deals in raw
///   bytes; RAII resources must be managed by the caller.
/// * Slots are guaranteed to satisfy the alignment requested through the [Layout].
///
/// ## Example
///
/// ```
/// use std::alloc::Layout;
/// use crate::rumtk_arena::mem::MemoryPool;
///
/// let mut pool = MemoryPool::new();
/// let layout = Layout::from_size_align(100, 16).unwrap();
/// let ptr = pool.allocate(layout);
/// assert!(!ptr.is_null());
/// unsafe { pool.deallocate(ptr, layout) };
/// ```
///
#[derive(Debug)]
pub struct MemoryPool {
    chunks: ChunkList,
    chunk_size: usize,
}

impl MemoryPool {
    ///
    /// Creates a new [MemoryPool] that grows in chunks of [DEFAULT_GLOBAL_MB_ALLOCATION] bytes.
    ///
    /// No memory is requested from the system until the first allocation arrives.
    ///
    pub const fn new() -> Self {
        Self::with_chunk_size(DEFAULT_GLOBAL_MB_ALLOCATION)
    }

    ///
    /// Creates a new [MemoryPool] that grows in chunks of `chunk_size` bytes.
    ///
    pub const fn with_chunk_size(chunk_size: usize) -> Self {
        Self {
            chunks: ChunkList::new_in(&DIRECT_ALLOCATOR),
            chunk_size,
        }
    }

    ///
    /// Rounds the [Layout] size up to the next power of two, never below [MIN_SLOT_SIZE].
    ///
    /// Returns [None] if the requested size cannot be rounded without overflowing.
    ///
    #[inline]
    pub fn slot_size(layout: &Layout) -> Option<usize> {
        let requested = layout.size();
        if requested <= MIN_SLOT_SIZE {
            Some(MIN_SLOT_SIZE)
        } else {
            requested.checked_next_power_of_two()
        }
    }

    #[inline(always)]
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    ///
    /// Number of chunks currently allocated by the pool.
    ///
    #[inline(always)]
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    ///
    /// Searches the allocated chunks for one with available space for the requested [Layout].
    ///
    /// Every visited [Chunk] is defragmented first so adjacent deallocated sections merge back
    /// into continuous memory before its availability is judged.
    ///
    #[inline]
    pub fn allocate_on_available(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        for chunk in self.chunks.iter_mut().rev() {
            match chunk.reclaim(size, align) {
                Some(ptr) => return Some(ptr),
                None => match chunk.allocate(size, align) {
                    Some(ptr) => return Some(ptr),
                    None => continue,
                },
            }
        }
        None
    }

    ///
    /// Obtains a `*mut u8` pointer to a slot of at least [Layout] size rounded up to the next
    /// power of two and aligned to the [Layout] alignment.
    ///
    /// Deallocated sections are defragmented and recycled first; a new [Chunk] is allocated only
    /// when no existing slot can serve the request. Returns a null pointer if the system is out
    /// of memory or the rounded size overflows.
    ///
    #[inline]
    pub fn allocate(&mut self, layout: Layout) -> *mut u8 {
        let size = match Self::slot_size(&layout) {
            Some(size) => size,
            None => return null_mut(),
        };
        let align = layout.align();

        match self.allocate_on_available(size, align) {
            Some(ptr) => ptr,
            None => {
                self.grow(size, align);
                match self.chunks.back_mut() {
                    Some(chunk) => chunk.allocate(size, align).unwrap_or(null_mut()),
                    None => null_mut(),
                }
            }
        }
    }

    ///
    /// Returns a slot to the pool. The owning [Chunk] records the slot as a section of continuous
    /// free memory that later allocations recycle. No memory is returned to the system.
    ///
    /// ## Safety
    ///
    /// `ptr` must have been obtained from [Self::allocate] on this pool with the same [Layout],
    /// and must not be used after this call. Unknown pointers are ignored, but double frees
    /// corrupt the bookkeeping and lead to overlapping allocations.
    ///
    #[inline]
    pub unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }
        let size = match Self::slot_size(&layout) {
            Some(size) => size,
            None => return,
        };
        if let Some(chunk) = self.chunks.iter_mut().rev().find(|chunk| chunk.contains(ptr)) {
            chunk.deallocate(ptr, size);
        }
    }

    ///
    /// Allocates a new [Chunk] of at least [Self::chunk_size] bytes. Requests bigger than the
    /// configured chunk size get a dedicated [Chunk] large enough to hold them.
    ///
    #[inline]
    fn grow(&mut self, size: usize, align: usize) {
        let needed = size.saturating_add(align);
        let capacity = if needed > self.chunk_size {
            needed
        } else {
            self.chunk_size
        };
        if let Some(chunk) = Chunk::new(capacity) {
            self.chunks.push_back(chunk);
        }
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for MemoryPool {}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(size: usize, align: usize) -> Layout {
        Layout::from_size_align(size, align).unwrap()
    }

    #[test]
    fn test_mempool_slot_size_is_power_of_two() {
        assert_eq!(MemoryPool::slot_size(&layout(1, 1)), Some(MIN_SLOT_SIZE));
        assert_eq!(MemoryPool::slot_size(&layout(16, 1)), Some(16));
        assert_eq!(MemoryPool::slot_size(&layout(17, 1)), Some(32));
        assert_eq!(MemoryPool::slot_size(&layout(1000, 1)), Some(1024));
        assert_eq!(MemoryPool::slot_size(&layout(1024, 1)), Some(1024));
    }

    #[test]
    fn test_mempool_allocates_usable_memory() {
        let mut pool = MemoryPool::with_chunk_size(1024);
        let ptr = pool.allocate(layout(100, 1));

        assert!(!ptr.is_null(), "Pool failed to allocate a slot!");
        assert_eq!(pool.chunk_count(), 1, "Pool did not allocate a chunk!");

        unsafe { std::ptr::write_bytes(ptr, 0xAB, 100) };
        let slice = unsafe { std::slice::from_raw_parts(ptr, 100) };
        assert!(slice.iter().all(|byte| *byte == 0xAB), "Slot memory is not usable!");
    }

    #[test]
    fn test_mempool_respects_alignment() {
        let mut pool = MemoryPool::with_chunk_size(1024);
        let ptr = pool.allocate(layout(24, 64));

        assert!(!ptr.is_null(), "Pool failed to allocate an aligned slot!");
        assert_eq!(ptr as usize % 64, 0, "Slot is not aligned to the requested alignment!");
    }

    #[test]
    fn test_mempool_allocates_new_chunk_when_full() {
        let mut pool = MemoryPool::with_chunk_size(64);
        let l = layout(64, 1);
        let first = pool.allocate(l);
        let second = pool.allocate(l);

        assert!(!first.is_null(), "Pool failed to allocate the first slot!");
        assert!(!second.is_null(), "Pool failed to allocate the second slot!");
        assert_ne!(first, second, "Pool handed out the same slot twice!");
        assert_eq!(pool.chunk_count(), 2, "Pool did not allocate a new chunk once full!");
    }

    #[test]
    fn test_mempool_allocates_dedicated_chunk_for_big_requests() {
        let mut pool = MemoryPool::with_chunk_size(64);
        let ptr = pool.allocate(layout(256, 1));

        assert!(!ptr.is_null(), "Pool failed to allocate a slot bigger than the chunk size!");
        unsafe { std::ptr::write_bytes(ptr, 0xCD, 256) };
    }
}
