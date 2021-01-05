pub trait DataStructure<T> {
    fn new() -> Self;
    fn add(&mut self, data: T);
    fn find(&self, item: T) -> Result<T, Box<dyn std::error::Error>>;
    fn remove(&self, item: T) -> bool;
}

