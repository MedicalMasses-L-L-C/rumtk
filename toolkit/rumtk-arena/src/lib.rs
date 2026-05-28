#![feature(allocator_api)]
#![feature(slice_ptr_get)]
extern crate core;

pub mod arena;

pub use arena::Arena;

#[cfg(test)]
mod tests {
    use crate::Arena;

    #[test]
    fn test_arena_simple_allocation() {
        let arena = Arena::new();
        let mut v = Vec::<usize, &Arena>::with_capacity_in(10, &arena);

        v.push(10);
        v.push(10);

        assert_eq!(v, [10, 10], "Failed to allocate and fill a small vector!");
    }

    #[test]
    fn test_arena_simple_reallocation() {
        let arena = Arena::new();
        let mut v = Vec::<usize, &Arena>::with_capacity_in(1, &arena);

        v.push(10);
        v.push(10);

        assert_eq!(v, [10, 10], "Failed to reallocate and fill a small vector!");
    }
}
