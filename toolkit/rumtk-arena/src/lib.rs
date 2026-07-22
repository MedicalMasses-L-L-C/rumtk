#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(liballoc_internals)]
#![feature(linked_list_retain)]
#![feature(portable_simd)]
extern crate alloc;
extern crate core;

pub mod arena;
pub mod collections;
pub mod dune;
pub mod constants;
pub mod buffers;
pub mod cpu;
pub mod mem;
mod global;
mod base;

pub use arena::Arena;

#[cfg(test)]
mod tests {
    use crate::collections::{ArenaHashMap, ArenaVec, ArenaVecDeque};
    use crate::{rumtk_arena_hashmap, rumtk_arena_vec, rumtk_arena_vecdeque, rumtk_dune_new, Arena};
    use std::alloc::Allocator;
    use std::alloc::Layout;
    use std::ptr::NonNull;


    macro_rules! rumtk_benchmark_snippet {
        ( $closure:expr ) => {{
            use std::time::Instant;

            let start = Instant::now();
            let r = $closure();
            let end = Instant::now();

            let time = end - start;
            let micros = time.as_micros();

            (r, micros)
        }};
    }

    #[test]
    fn test_arena_simple_allocation() {
        let arena = Arena::with_capacity(1024);
        let v: &str = unsafe { arena.write("hello world").unwrap().as_ref() };

        assert_eq!(v, "hello world", "Failed to allocate and fill a small vector!");
    }

    #[test]
    fn test_arena_simple_reallocation_address() {
        let arena = Arena::with_capacity(1024);
        let old_layout = Layout::from_size_align(4, 1).unwrap();
        let new_layout = Layout::from_size_align(8, 1).unwrap();
        let v = unsafe { arena.allocate(old_layout).unwrap() };
        let v2 = unsafe { arena.grow(v.cast(), old_layout, new_layout).unwrap() };
        let v3 = unsafe { arena.grow(v2.cast(), new_layout, new_layout).unwrap() };

        assert_eq!(v2.addr(), v3.addr(), "Failed to reallocate without invalidating pointer!");
    }

    #[test]
    fn test_arena_simple_reallocation() {
        let arena = Arena::with_capacity(1024);
        let old_layout = Layout::from_size_align(4, 4).unwrap();
        let new_layout = Layout::from_size_align(8, 4).unwrap();
        let v: NonNull<[u8]> = unsafe { arena.allocate(old_layout).unwrap() };
        let v2: NonNull<[u8]> = unsafe { arena.grow(v.cast(), old_layout, new_layout).unwrap() };

        assert_eq!(v.addr(), v2.addr(), "Failed to reallocate without invalidating pointer!");
    }

    #[test]
    fn test_arena_simple_vec_allocation() {
        let arena = Arena::with_capacity(1024);
        let mut v = Vec::<usize, &Arena>::with_capacity_in(10, &arena);

        v.push(10);
        v.push(10);

        assert_eq!(v, [10, 10], "Failed to allocate and fill a small vector!");
    }

    #[test]
    fn test_arena_simple_vec_reallocation() {
        let arena = Arena::with_capacity(1024);
        let mut v = Vec::<usize, &Arena>::with_capacity_in(1, &arena);

        v.push(10);
        v.push(10);

        assert_eq!(v, [10, 10], "Failed to reallocate and fill a small vector!");
    }

    #[test]
    fn test_arena_allocate_more_than_allowed() {
        let arena = Arena::with_capacity(5);
        let v = arena.commit(10);

        assert!(v.is_err(), "Arena did not emit error upon allocation of byte count higher than current capacity.");
    }

    #[test]
    fn test_arena_create_vec_with_macro() {
        let arena = Arena::with_capacity(5);
        let v: ArenaVec<String> = rumtk_arena_vec!(&arena);

        assert!(v.is_empty(), "Failed to create vector with arena allocation enabled.");
    }

    #[test]
    fn test_arena_benchmark_arenavec_vs_vec() {
        struct ptr {
            data: usize,
            len: usize,
            index: usize,
            bad: usize,
        }

        impl ptr {
            pub fn new() -> Self {
                Self {
                    data: 0,
                    len: 0,
                    index: 0,
                    bad: 0,
                }
            }
        }

        let total_items = 20000;

        let (arena, arena_time) = rumtk_benchmark_snippet!(|| {
            let total_bytes = (total_items * size_of::<ptr>()) + size_of::<ArenaVec<ptr>>();
            Arena::with_capacity(total_bytes)
        });

        let (arena_vec_r, arena_vec_time) = rumtk_benchmark_snippet!(|| {
            let mut v: ArenaVec<ptr> = rumtk_arena_vec!(&arena);

            for _ in 0..total_items {
                v.push(ptr::new());
            }

            v
        });

        let (vec_r, vec_time) = rumtk_benchmark_snippet!(|| {
            let mut v = Vec::<ptr>::with_capacity(total_items);

            for _ in 0..total_items {
                v.push(ptr::new());
            }

            v
        });

        let total_arena_vec_time = arena_time + arena_vec_time;
        println!("ArenaVec => {} us vs. Vec => {} us.", total_arena_vec_time, vec_time);

        //assert!(total_arena_vec_time < vec_time, "ArenaVec is too slow. ArenaVec => {} us vs. Vec => {} us.", total_arena_vec_time, vec_time);
    }

    #[test]
    fn test_arena_create_vec_with_macro_with_items() {
        let arena = Arena::with_capacity(50);
        let expected = &["Hello", "World", "!"];
        let v: ArenaVec<&str> = rumtk_arena_vec!(expected.clone(), &arena);

        assert_eq!(v.as_slice(), expected, "Failed to create vector with arena allocation enabled and item slice.");
    }

    #[test]
    fn test_arena_create_vecdeque_with_macro() {
        let arena = Arena::with_capacity(5);
        let v: ArenaVecDeque<String> = rumtk_arena_vecdeque!(&arena);

        assert!(v.is_empty(), "Failed to create vector with arena allocation enabled.");
    }

    #[test]
    fn test_arena_create_vecdeque_with_macro_with_items() {
        let arena = Arena::with_capacity(50);
        let expected = ["Hello", "World", "!"];
        let mut v: ArenaVecDeque<&str> = rumtk_arena_vecdeque!(expected.clone(), &arena);

        assert_eq!(v.pop_front(), Some(expected[0]), "Failed to create queue with arena allocation enabled and item slice.");
    }

    #[test]
    fn test_arena_create_hashmap_with_macro() {
        let arena = Arena::with_capacity(5);
        let v: ArenaHashMap<&str, &str> = rumtk_arena_hashmap!(&arena);

        assert!(v.is_empty(), "Failed to create vector with arena allocation enabled.");
    }

    #[test]
    fn test_arena_create_hashmap_with_macro_with_items() {
        let arena = Arena::with_capacity(120);
        let expected = [(0, "Hello"), (1, "World"), (2, "!")];
        let v: ArenaHashMap<usize, &str> = rumtk_arena_hashmap!(expected.clone(), &arena);

        assert_eq!(v[&0], expected[0].1, "Failed to create hashmap with arena allocation enabled and item slice.");
    }

    #[test]
    fn test_arena_vec_debug_print() {
        let arena = rumtk_dune_new!(500);
        let mut test_vec = ArenaVec::new_in(arena.arena);
        let expected = ["Hello", "World", "!"];

        for s in expected.iter() {
            test_vec.push(s);
        }

        println!("{:?}", &test_vec);
    }

    #[test]
    fn test_arena_map_debug_print() {
        let arena = rumtk_dune_new!(500);
        let expected = [(5, "Hello"), (1, "World"), (3, "!")];


        let v = rumtk_arena_hashmap!(expected.clone(), &arena.arena);
        println!("{:?}", &v);
    }
}
