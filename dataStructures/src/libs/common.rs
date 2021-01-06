#![allow(non_snake_case)]
pub trait DataStructure<T> {
    fn new() -> Self;
    fn add(&mut self, data: T);
    fn findItem(&mut self, item: T) -> Result<u64, Box<dyn std::error::Error>>;
    fn remove(&mut self, item: T) -> Result<bool, Box<dyn std::error::Error>>;
}
