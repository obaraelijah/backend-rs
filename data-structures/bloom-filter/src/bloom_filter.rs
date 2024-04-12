pub trait BloomFilter {
    fn insert(&mut self, key: &str);

    /// probably yes, definitely no.
    fn contains(&self, key: &str) -> bool;
}