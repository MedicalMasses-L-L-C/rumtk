pub mod components;
pub mod pages;
pub mod static_components;
pub mod utils;

///
/// Add utils unit tests here to ensure internal functions work.
///
#[cfg(test)]
mod tests {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
