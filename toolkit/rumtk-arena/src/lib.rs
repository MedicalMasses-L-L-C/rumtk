#![feature(allocator_api)]
pub mod arena;

pub use arena::Arena;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
