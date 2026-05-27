#![feature(allocator_api)]
#![feature(slice_ptr_get)]
pub mod arena;

pub use arena::Arena;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
