#![feature(allocator_api)]
#![feature(slice_ptr_get)]
extern crate core;

pub mod arena;

pub use arena::Arena;

#[cfg(test)]
mod tests {
    use crate::Arena;
    use std::alloc::Allocator;
    use std::alloc::Layout;
    use std::ptr::NonNull;

    #[test]
    fn test_arena_simple_allocation() {
        let arena = Arena::new();
        let v: &str = unsafe { arena.write("hello world").unwrap().as_ref() };

        assert_eq!(v, "hello world", "Failed to allocate and fill a small vector!");
    }

    #[test]
    fn test_arena_simple_reallocation_address() {
        let arena = Arena::new();
        let old_layout = Layout::from_size_align(4, 1).unwrap();
        let new_layout = Layout::from_size_align(8, 1).unwrap();
        let v = unsafe { arena.allocate(old_layout).unwrap() };
        let v2 = unsafe { arena.grow(v.cast(), old_layout, new_layout).unwrap() };
        let v3 = unsafe { arena.grow(v2.cast(), new_layout, new_layout).unwrap() };

        assert_eq!(v2, v3, "Failed to reallocate without invalidating pointer!");
    }

    #[test]
    fn test_arena_simple_reallocation() {
        let arena = Arena::new();
        let old_layout = Layout::from_size_align(4, 4).unwrap();
        let new_layout = Layout::from_size_align(8, 4).unwrap();
        let v: NonNull<[u8]> = unsafe { arena.allocate(old_layout).unwrap() };
        let v2: NonNull<[u8]> = unsafe { arena.grow(v.cast(), old_layout, new_layout).unwrap() };

        assert_eq!(v.addr(), v2.addr(), "Failed to reallocate without invalidating pointer!");
    }

    #[test]
    fn test_arena_simple_vec_allocation() {
        let arena = Arena::new();
        let mut v = Vec::<usize, &Arena>::with_capacity_in(10, &arena);

        v.push(10);
        v.push(10);

        assert_eq!(v, [10, 10], "Failed to allocate and fill a small vector!");
    }

    #[test]
    fn test_arena_simple_vec_reallocation() {
        let arena = Arena::new();
        let mut v = Vec::<usize, &Arena>::with_capacity_in(1, &arena);

        v.push(10);
        v.push(10);

        assert_eq!(v, [10, 10], "Failed to reallocate and fill a small vector!");
    }

    #[test]
    fn test_arena_sallocate_more_than_allowed() {
        let arena = Arena::with_capacity(5);
        let v = arena.commit(10);

        assert!(v.is_err(), "Arena did not emit error upon allocation of byte count higher than current capacity.");
    }
}
